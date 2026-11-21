#!/usr/bin/env bash
set -euo pipefail

FILE="core/src/config/database.rs"

DRIVER="$(printf '%s' "${DB_CONNECTION:-}" | tr '[:upper:]' '[:lower:]')"

case "$DRIVER" in
    mysql)
        POOL="MySqlPool"
        CREATOR="create_db_pool_mysql"
        ;;
    pgsql|postgres|postgresql)
        POOL="PgPool"
        CREATOR="create_db_pool_pgsql"
        ;;
    sqlite)
        POOL="SqlitePool"
        CREATOR="create_db_pool_sqlite"
        ;;
    *)
        echo "::error::Unsupported DB_CONNECTION: ${DRIVER:-unset}"
        exit 1
        ;;
esac

if [[ ! -f "$FILE" ]]; then
    echo "::error::File not found: $FILE"
    exit 1
fi

perl - "$FILE" "$POOL" "$CREATOR" "$DRIVER" <<'PERL'
use strict;
use warnings;

my ($file, $pool, $creator, $driver) = @ARGV;

open my $fh, '<', $file
    or die "Cannot read $file: $!";

local $/;
my $source = <$fh>;
close $fh;

my $original = $source;

# 1. Locate and replace DbPoolType.
my $pool_pattern =
    qr/(pub\s+type\s+DbPoolType\s*=\s*sqlx::)([A-Za-z_][A-Za-z0-9_]*)(\s*;)/;

my $pool_count = () = $source =~ /$pool_pattern/g;

die "Expected exactly one DbPoolType declaration; found $pool_count\n"
    unless $pool_count == 1;

if ($source =~ /$pool_pattern/) {
    my ($prefix, $old_pool, $suffix) = ($1, $2, $3);

    substr(
        $source,
        $-[0],
        $+[0] - $-[0],
        $prefix . $pool . $suffix
    );
}

# 2. Locate create_db_pool() and its creator call.
my $function_pattern = qr/
    (pub\s+async\s+fn\s+create_db_pool\s*
    \(\s*\)\s*->\s*Result\s*<[^>]+>\s*\{)
    (.*?)
    (\n\})
/sx;

my $function_count = () = $source =~ /$function_pattern/g;

die "Expected exactly one create_db_pool() function; found $function_count\n"
    unless $function_count == 1;

if ($source =~ /$function_pattern/) {
    my ($header, $body, $footer) = ($1, $2, $3);

    my $call_pattern = qr/
        \b(create_db_pool_[A-Za-z0-9_]*)
        (\s*\(\s*\)\s*\.await)
    /x;

    my $call_count = () = $body =~ /$call_pattern/g;

    die "Expected exactly one creator call; found $call_count\n"
        unless $call_count == 1;

    $body =~ s/$call_pattern/$creator$2/;

    my $replacement = $header . $body . $footer;

    substr(
        $source,
        $-[0],
        $+[0] - $-[0],
        $replacement
    );
}

die "No changes detected\n"
    if $source eq $original;

# 3. Verify the resulting configuration before writing.
die "Incorrect pool type after replacement\n"
    unless $source =~ /pub\s+type\s+DbPoolType\s*=\s*sqlx::\Q$pool\E\s*;/;

die "Incorrect creator after replacement\n"
    unless $source =~ /pub\s+async\s+fn\s+create_db_pool\s*\(\s*\)[\s\S]*?\b\Q$creator\E\s*\(\s*\)\s*\.await/;

# 4. Write only after every validation succeeds.
open my $out, '>', $file
    or die "Cannot write $file: $!";

print {$out} $source;
close $out;

print "Database pool adapted successfully\n";
print "Driver: $driver\n";
print "Pool: sqlx::$pool\n";
print "Creator: $creator\n";
PERL

echo "Database configuration updated for $DRIVER"
