
## CSRF in multipart form data:

Multipart token validation: The token must be the first field in the multipart request. If it is not, the request is rejected with a 419 response (fail-closed). This requirement helps reduce the processing overhead of parsing multipart requests.

The peek_multipart function reads body chunks only until the first multipart field is complete. It then restores the request body by combining the bytes already read with the remaining stream. Only the initial few kilobytes are buffered, so large uploads continue to stream without being fully loaded into memory.

[csrf.rs](../core/src/http/csrf.rs)
---------


## Schema: Fetch Table and Column Names in a Single Query

**Previous code:**

```rust
let table = self.fix_table_name(table);
let tables = get_tables().await;
if !tables.contains(&table) {
    logger::error(&format!("table not exists {}", table));
    logger::info("May you need to run migrations or restart app to re-cache tables!?");
    return None;
}
let columns = get_columns().await;
for col in [column, except_col] {
    if !columns.contains(&format!("{}.{}", table, col)) {
        logger::error(&format!("column `{}` not exists in {}", col, table));
        logger::info("May you need to run migrations or restart app to re-cache tables!?");
        return None;
    }
}
```

**New code:**

```rust
let tables_columns = self.get_db_tables_columns_names();
match tables_columns.await {
    Ok(columns) => {
        if !columns.contains(&format!("{}.{}", table, column)) {
            logger::error(&format!("table or column `{}` not exists in {}", column, table));
            logger::info("May you need to run migrations or restart app to re-cache tables!?");
            return None;
        }
    },
    Err(_e) => {
        return None;
    }
}
```

The issue was that these functions had already been implemented, and we were reusing them to preserve backward compatibility. Although the fetched table and column values were cached, the implementation required separate queries. Fetching everything in a single query is a better approach, and using `Arc` makes cloning the result inexpensive.

## use native db pool

To use native db pool we need change: `core/src/config/database.rs`:

- default is any:

```rust
// define default db pool
pub type DbPoolType = sqlx::AnyPool;
// define default pool creator function
pub async fn create_db_pool() -> Result<DbPoolType, DbError> {
    create_db_pool_any().await
```

- for example mysql native:

```rust
pub type DbPoolType = sqlx::MySqlPool;          // change here

pub async fn create_db_pool() -> Result<DbPoolType, DbError> {
    create_db_pool_mysql().await                 // and here
}
```

## faster compile

Optional: Speed Up Compilation with sccache

If you're tired of waiting for Rust compilation, you can use sccache to cache compiler outputs and speed up subsequent builds.

Install sccache:
```bash
cargo install sccache
```

Open your Cargo configuration file:
```bash
~/.cargo/config.toml
```

Or Uncomment the following lines in `.cargo/config.toml`
```toml
[build]
rustc-wrapper = "sccache"
```

That's it! Cargo will automatically use sccache for your builds.

Note: This is completely optional. You can build the project normally without sccache.


## How use resource & paginator each other

```rust

/// Logs a database error and maps it to a 500 response.
fn db_error(e: sqlx::Error) -> StatusCode {
    // Keep the real error on the server side only.
    println!("{e:#?}");
    StatusCode::INTERNAL_SERVER_ERROR
}

/// Returns one page of open todos as Laravel-style pagination JSON.
async fn index(/* State, Query, current_url ... */) -> Result<impl IntoResponse, StatusCode> {
    // Total matching rows; needed for `total` and `last_page`.
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM todos WHERE deleted_at IS NULL")
        .fetch_one(&pool)
        .await
        .map_err(db_error)?;

    let paginator = PaginateBuilder::new(per_page, count as usize, page, current_url);

    // LIMIT and OFFSET both come from the builder, so they use the clamped values.
    let rows = sqlx::query_as::<_, TodoRow>(
        "SELECT id, title FROM todos WHERE deleted_at IS NULL ORDER BY id LIMIT ? OFFSET ?",
    )
    .bind(paginator.limit() as i64)
    .bind(paginator.offset() as i64)
    .fetch_all(&pool)
    .await
    .map_err(db_error)?;

    // Wrap each row in its resource; the paginator only needs `Serialize` items.
    let data = rows.into_iter().map(TodoResource::from).collect::<Vec<_>>();

    Ok(Json(paginator.build(data)))
}
```