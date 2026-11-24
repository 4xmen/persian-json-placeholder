#!/usr/bin/env bash
set -euo pipefail

FILE="${DATABASE_CONFIG_FILE:-core/src/config/database.rs}"

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

# -----------------------------------------
# 1. Replace DbPoolType
# -----------------------------------------

my $pool_pattern = qr{
    (pub\s+type\s+DbPoolType\s*=\s*sqlx::)
    ([A-Za-z_][A-Za-z0-9_]*)
    (\s*;)
}x;

my $pool_count = 0;

$source =~ s{
    $pool_pattern
}{
    $pool_count++;
    $1 . $pool . $3;
}gex;

die "Expected exactly one DbPoolType declaration; found $pool_count\n"
    unless $pool_count == 1;

# -----------------------------------------
# 2. Find create_db_pool()
# -----------------------------------------

my $function_pattern = qr{
    (pub\s+async\s+fn\s+create_db_pool\s*
     \(\s*\)\s*->\s*Result\s*<[^>]+>\s*\{)
    (.*?)
    (\n\})
}sx;

die "create_db_pool() function not found\n"
    unless $source =~ /$function_pattern/;

my ($function_start, $function_end) = ($-[0], $+[0]);

my ($header, $body, $footer) = ($1, $2, $3);

# Make sure there is no second matching function.
my $remaining = substr($source, $function_end);

die "Multiple create_db_pool() functions found\n"
    if $remaining =~ /$function_pattern/;

# -----------------------------------------
# 3. Replace creator call
# -----------------------------------------

my $call_pattern = qr{
    \b(create_db_pool_[A-Za-z0-9_]*)
    (\s*\(\s*\)\s*\.await)
}x;

my $call_count = 0;

$body =~ s{
    $call_pattern
}{
    $call_count++;
    $creator . $2;
}gex;

die "Expected exactly one creator call; found $call_count\n"
    unless $call_count == 1;

my $new_function = $header . $body . $footer;

substr(
    $source,
    $function_start,
    $function_end - $function_start,
    $new_function
);

# -----------------------------------------
# 4. Validate everything before writing
# -----------------------------------------

die "Incorrect pool type after replacement\n"
    unless $source =~ /pub\s+type\s+DbPoolType\s*=\s*sqlx::\Q$pool\E\s*;/;

die "Incorrect creator after replacement\n"
    unless $source =~ /pub\s+async\s+fn\s+create_db_pool\s*\(\s*\)[\s\S]*?\b\Q$creator\E\s*\(\s*\)\s*\.await/;

die "No changes detected\n"
    if $source eq $original;

# -----------------------------------------
# 5. Write only after successful validation
# -----------------------------------------

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
