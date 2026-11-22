#!/usr/bin/env python3

import os
import re
import sys
from pathlib import Path


DRIVERS = {
    "mysql": {
        "pkey": "u64",
        "pool": "MySqlPool",
        "creator": "create_db_pool_mysql",
    },
    "pgsql": {
        "pkey": "i64",
        "pool": "PgPool",
        "creator": "create_db_pool_pgsql",
    },
    "postgres": {
        "pkey": "i64",
        "pool": "PgPool",
        "creator": "create_db_pool_pgsql",
    },
    "postgresql": {
        "pkey": "i64",
        "pool": "PgPool",
        "creator": "create_db_pool_pgsql",
    },
    "sqlite": {
        "pkey": "i64",
        "pool": "SqlitePool",
        "creator": "create_db_pool_sqlite",
    },
}


def fail(message: str) -> None:
    print(f"::error::{message}")
    sys.exit(1)


def main() -> None:
    driver = os.environ.get("DB_CONNECTION", "").strip().lower()

    if driver not in DRIVERS:
        fail(f"Unsupported DB_CONNECTION: {driver!r}")

    config = DRIVERS[driver]

    pkey = config["pkey"]
    pool = config["pool"]
    creator = config["creator"]

    file_path = Path(
        os.environ.get(
            "DATABASE_CONFIG_FILE",
            "core/src/config/database.rs",
        )
    )

    if not file_path.is_file():
        fail(f"Database configuration not found: {file_path}")

    try:
        source = file_path.read_text(encoding="utf-8")
    except OSError as exc:
        fail(f"Cannot read database configuration: {exc}")

    original = source

    # --------------------------------------------------
    # 1. Replace PKeyType
    # --------------------------------------------------

    pkey_pattern = re.compile(
        r"(?m)^([ \t]*pub[ \t]+type[ \t]+PKeyType"
        r"[ \t]*=[ \t]*)"
        r"[A-Za-z_][A-Za-z0-9_]*"
        r"([ \t]*;)"
    )

    source, pkey_count = pkey_pattern.subn(
        lambda match: match.group(1) + pkey + match.group(2),
        source,
    )

    if pkey_count != 1:
        fail(
            "Expected exactly one PKeyType declaration, "
            f"found {pkey_count}"
        )

    # --------------------------------------------------
    # 2. Replace DbPoolType
    # --------------------------------------------------

    pool_pattern = re.compile(
        r"(?m)^([ \t]*pub[ \t]+type[ \t]+DbPoolType"
        r"[ \t]*=[ \t]*sqlx::)"
        r"([A-Za-z_][A-Za-z0-9_]*)"
        r"([ \t]*;)"
    )

    source, pool_count = pool_pattern.subn(
        lambda match: match.group(1) + pool + match.group(3),
        source,
    )

    if pool_count != 1:
        fail(
            "Expected exactly one DbPoolType declaration, "
            f"found {pool_count}"
        )

    # --------------------------------------------------
    # 3. Locate create_db_pool()
    # --------------------------------------------------

    function_pattern = re.compile(
        r"(?m)^([ \t]*pub[ \t]+async[ \t]+fn"
        r"[ \t]+create_db_pool[ \t]*"
        r"\([ \t]*\)[ \t]*->[ \t]*Result"
        r"[ \t]*<[^\n]+>[ \t]*\{)"
        r"([\s\S]*?)"
        r"^([ \t]*\})"
    )

    functions = list(function_pattern.finditer(source))

    if len(functions) != 1:
        fail(
            "Expected exactly one create_db_pool() function, "
            f"found {len(functions)}"
        )

    function = functions[0]
    body = function.group(2)

    # --------------------------------------------------
    # 4. Replace the existing creator call
    # --------------------------------------------------

    creator_pattern = re.compile(
        r"\b(create_db_pool_[A-Za-z0-9_]*)"
        r"([ \t]*\([ \t]*\)[ \t]*\.await)"
    )

    body, creator_count = creator_pattern.subn(
        lambda match: creator + match.group(2),
        body,
    )

    if creator_count != 1:
        fail(
            "Expected exactly one pool creator call, "
            f"found {creator_count}"
        )

    replacement = (
            function.group(1)
            + body
            + function.group(3)
    )

    source = (
            source[:function.start()]
            + replacement
            + source[function.end():]
    )

    # --------------------------------------------------
    # 5. Validate the resulting source
    # --------------------------------------------------

    expected_pkey = re.compile(
        r"(?m)^[ \t]*pub[ \t]+type[ \t]+PKeyType"
        r"[ \t]*=[ \t]*"
        + re.escape(pkey)
        + r"[ \t]*;"
    )

    if len(expected_pkey.findall(source)) != 1:
        fail("PKey type validation failed")

    expected_pool = re.compile(
        r"(?m)^[ \t]*pub[ \t]+type[ \t]+DbPoolType"
        r"[ \t]*=[ \t]*sqlx::"
        + re.escape(pool)
        + r"[ \t]*;"
    )

    if len(expected_pool.findall(source)) != 1:
        fail("Pool type validation failed")

    expected_creator = re.compile(
        r"\b" + re.escape(creator)
        + r"[ \t]*\([ \t]*\)[ \t]*\.await"
    )

    if len(expected_creator.findall(source)) != 1:
        fail("Pool creator validation failed")

    if source == original:
        print("Database configuration already matches the selected driver.")
        return

    # --------------------------------------------------
    # 6. Write only after successful validation
    # --------------------------------------------------

    try:
        file_path.write_text(source, encoding="utf-8")
    except OSError as exc:
        fail(f"Cannot write database configuration: {exc}")

    print("Database configuration adapted successfully!")
    print(f"Driver : {driver}")
    print(f"PKey   : {pkey}")
    print(f"Pool   : sqlx::{pool}")
    print(f"Creator: {creator}")
    print(f"File   : {file_path}")


if __name__ == "__main__":
    main()
