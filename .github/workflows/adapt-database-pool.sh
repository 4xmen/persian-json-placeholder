#!/usr/bin/env bash

set -euo pipefail

FILE="core/src/config/database.rs"

case "${DB_CONNECTION,,}" in
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
        echo "::error::Unsupported DB_CONNECTION: ${DB_CONNECTION:-unset}"
        exit 1
        ;;
esac

if [[ ! -f "$FILE" ]]; then
    echo "::error::File not found: $FILE"
    exit 1
fi

perl - "$FILE" "$POOL" "$CREATOR" <<'PERL'
use strict;
use warnings;

my ($file, $pool, $creator) = @ARGV;

open my $fh, '<', $file or die "Cannot read $file: $!";
local $/;
my $source = <$fh>;
close $fh;

my $original = $source;

# Replace DbPoolType.
my $pool_count = $source =~ s{
    (pub\s+type\s+DbPoolType\s*=\s*sqlx::)
    [A-Za-z_][A-Za-z0-9_]*
    (\s*;)
}{$1$pool$2}gx;

die "Expected exactly one DbPoolType declaration\n"
    unless $pool_count == 1;

# Locate create_db_pool() and its current creator.
my $function_count = 0;

$source =~ s{
    (pub\s+async\s+fn\s+create_db_pool\s*
     \(\s*\)\s*->\s*Result\s*<[^>]+>\s*\{)
    ([\s\S]*?)
    (\n\})
}{
    my ($head, $body, $tail) = ($1, $2, $3);

    my $count = $body =~ s{
        \b(create_db_pool_[A-Za-z0-9_]*)
        (\s*\(\s*\)\s*\.await)
    }{$creator$2}gx;

    die "Expected exactly one creator call\n" unless $count == 1;

    $function_count++;
    $head . $body . $tail;
}gex;

die "Expected exactly one create_db_pool function\n"
    unless $function_count == 1;

# Replace the imported creator.
my $import_count = $source =~ s{
    (use\s+crate::sql::database_client::\{)
    ([^}]*)
    (\};)
}{
    my ($head, $imports, $tail) = ($1, $2, $3);

    my @items = map { s/^\s+|\s+$//gr }
                grep { /\S/ }
                split /,/, $imports;

    my $found = 0;

    for (@items) {
        if (/^create_db_pool_[A-Za-z0-9_]*$/) {
            $_ = $creator;
            $found++;
        }
    }

    die "Expected exactly one creator import\n" unless $found == 1;

    $head . join(", ", @items) . $tail;
}gex;

die "Expected exactly one database client import\n"
    unless $import_count == 1;

die "No changes detected\n" if $source eq $original;

open my $out, '>', $file or die "Cannot write $file: $!";
print {$out} $source;
close $out;

print "Database pool adapted successfully\n";
print "Driver: $ENV{DB_CONNECTION}\n";
print "Pool: sqlx::$pool\n";
print "Creator: $creator\n";
PERL
