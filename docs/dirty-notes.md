
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