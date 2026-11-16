use crate::config::CONFIG;
use crate::config::database::DatabaseEngine;
use crate::logger;
use sqlx::any::install_default_drivers;
use sqlx::postgres::PgConnectOptions;
use sqlx::{AnyPool, AssertSqlSafe, MySqlPool, PgPool, Row, SqlitePool};
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub enum DbError {
    Sqlx(sqlx::Error),
    InvalidTable,
    NotFound,
    InvalidQuery(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Sqlx(e) => write!(f, "sqlx error: {e}"),
            DbError::InvalidTable => write!(f, "invalid table"),
            DbError::NotFound => write!(f, "not found"),
            DbError::InvalidQuery(q) => write!(f, "invalid query: {q}"),
        }
    }
}

impl std::error::Error for DbError {}

#[async_trait::async_trait]
pub trait DatabaseClient: Send + Sync + Debug {
    async fn execute(&self, sql: &str) -> Result<(), DbError>;
    async fn execute_params(&self, sql: &str, params: &[&str]) -> Result<(), DbError>;
    async fn fetch_strings(&self, sql: &str) -> Result<Vec<String>, DbError>;
    async fn fetch_strings_params(
        &self,
        sql: &str,
        params: &[&str],
    ) -> Result<Vec<String>, DbError>;
    async fn fetch_count_params(&self, sql: &str, params: &[&str]) -> Result<i64, DbError>;
    async fn fetch_numbers(&self, sql: &str) -> Result<Vec<i64>, DbError>;
    /// Runs every statement in order on one pooled connection. Needed for
    /// session settings such as `SET FOREIGN_KEY_CHECKS = 0`.
    async fn execute_on_one_connection(&self, statements: &[String]) -> Result<(), DbError>;
}

#[derive(Debug)]
pub struct MySqlClient {
    pub pool: sqlx::MySqlPool,
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        DbError::Sqlx(err)
    }
}

#[async_trait::async_trait]
impl DatabaseClient for MySqlClient {
    async fn execute_on_one_connection(&self, statements: &[String]) -> Result<(), DbError> {
        let mut conn = self.pool.acquire().await?;
        for statement in statements {
            for single_sql in split_sql_statements(statement) {
                sqlx::query(AssertSqlSafe(single_sql))
                    .execute(&mut *conn)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_params(
        &self,
        sql: &str,
        params: &[&str], // ← str reference
    ) -> Result<(), DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn execute(&self, sql: &str) -> Result<(), DbError> {
        // for single_sql in sql.split(";") {
        //     if !single_sql.trim().is_empty() {
        //         sqlx::query(AssertSqlSafe(single_sql)).execute(&self.pool).await?;
        //     }
        // }

        for single_sql in split_sql_statements(sql) {
            sqlx::query(AssertSqlSafe(single_sql))
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    async fn fetch_strings(&self, sql: &str) -> Result<Vec<String>, DbError> {
        let rows = sqlx::query(AssertSqlSafe(sql))
            .fetch_all(&self.pool)
            .await?;

        if rows.is_empty() {
            return Ok(vec![]);
        }
        Ok(rows
            .into_iter()
            .map(|row| {
                // dbg!(&row);
                row.get::<String, _>(0)
            })
            .collect())
    }

    async fn fetch_numbers(&self, sql: &str) -> Result<Vec<i64>, DbError> {
        let rows = sqlx::query(AssertSqlSafe(sql))
            .fetch_all(&self.pool)
            .await?;

        // MAX() on an empty table returns a single NULL row, so decode as Option
        let mut numbers = Vec::with_capacity(rows.len());
        for row in rows {
            if let Some(n) = row.try_get::<Option<i64>, _>(0)? {
                numbers.push(n);
            }
        }
        Ok(numbers)
        // if rows.is_empty() {
        //     return Ok(vec![]);
        // }
        //
        // Ok(rows
        //     .into_iter()
        //     .map(|row| {
        //         // dbg!(&row);
        //         row.get::<i64, _>(0)
        //     })
        //     .collect())
    }

    async fn fetch_strings_params(
        &self,
        sql: &str,
        params: &[&str],
    ) -> Result<Vec<String>, DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        let rows = query.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                // dbg!(&row);
                row.get::<String, _>(0)
            })
            .collect())
    }

    async fn fetch_count_params(&self, sql: &str, params: &[&str]) -> Result<i64, DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        let row = query.fetch_one(&self.pool).await?;

        let count: i64 = row.try_get(0)?;

        Ok(count)
    }
}

#[derive(Debug)]
pub struct SqliteClient {
    pub pool: sqlx::SqlitePool,
}

#[async_trait::async_trait]
impl DatabaseClient for SqliteClient {
    async fn execute_on_one_connection(&self, statements: &[String]) -> Result<(), DbError> {
        let mut conn = self.pool.acquire().await?;
        for statement in statements {
            for single_sql in split_sql_statements(statement) {
                sqlx::query(AssertSqlSafe(single_sql))
                    .execute(&mut *conn)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_params(
        &self,
        sql: &str,
        params: &[&str], // ← str reference
    ) -> Result<(), DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn execute(&self, sql: &str) -> Result<(), DbError> {
        // for single_sql in sql.split(";") {
        //     if !single_sql.trim().is_empty() {
        //         sqlx::query(AssertSqlSafe(single_sql)).execute(&self.pool).await?;
        //     }
        // }

        for single_sql in split_sql_statements(sql) {
            sqlx::query(AssertSqlSafe(single_sql))
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    async fn fetch_strings(&self, sql: &str) -> Result<Vec<String>, DbError> {
        let rows = sqlx::query(AssertSqlSafe(sql))
            .fetch_all(&self.pool)
            .await?;
        if rows.is_empty() {
            return Ok(vec![]);
        }
        Ok(rows
            .into_iter()
            .map(|row| row.get::<String, _>(0))
            .collect())
    }

    async fn fetch_numbers(&self, sql: &str) -> Result<Vec<i64>, DbError> {
        let rows = sqlx::query(AssertSqlSafe(sql))
            .fetch_all(&self.pool)
            .await?;

        // MAX() on an empty table returns a single NULL row, so decode as Option
        let mut numbers = Vec::with_capacity(rows.len());
        for row in rows {
            if let Some(n) = row.try_get::<Option<i64>, _>(0)? {
                numbers.push(n);
            }
        }
        Ok(numbers)

        // if rows.is_empty() {
        //     return Ok(vec![]);
        // }
        // Ok(rows
        //     .into_iter()
        //     .map(|row| row.get::<i64, _>(0))
        //     .collect())
    }

    async fn fetch_strings_params(
        &self,
        sql: &str,
        params: &[&str],
    ) -> Result<Vec<String>, DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        let rows = query.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                // dbg!(&row);
                row.get::<String, _>(0)
            })
            .collect())
    }

    async fn fetch_count_params(&self, sql: &str, params: &[&str]) -> Result<i64, DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        let row = query.fetch_one(&self.pool).await?;

        let count: i64 = row.try_get(0)?;

        Ok(count)
    }
}

/// Splits a SQL script into single statements on `;`.
///
/// A `;` inside quotes ('...', "...", `...`) or comments (`-- ...`, `/* ... */`)
/// does not end a statement. Comments are dropped from the output.
pub fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut chars = sql.chars().peekable();

    while let Some(c) = chars.next() {
        if let Some(q) = quote {
            current.push(c);
            if c == '\\' && q != '`' {
                // escaped char inside a string, e.g. 'It\'s'
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            } else if c == q {
                quote = None;
            }
            continue;
        }

        match c {
            '\'' | '"' | '`' => {
                quote = Some(c);
                current.push(c);
            }
            '-' if chars.peek() == Some(&'-') => while chars.next_if(|&n| n != '\n').is_some() {},
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut prev = '\0';
                for n in chars.by_ref() {
                    if prev == '*' && n == '/' {
                        break;
                    }
                    prev = n;
                }
            }
            ';' => {
                if !current.trim().is_empty() {
                    statements.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }

    if !current.trim().is_empty() {
        statements.push(current.trim().to_string());
    }
    statements
}

#[derive(Debug)]
pub struct PostgresClient {
    pub pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl DatabaseClient for PostgresClient {
    /// Runs all statements on ONE connection inside ONE transaction.
    /// PostgreSQL DDL is transactional (Laravel also wraps Postgres migrations in a
    /// transaction), so a failed migration is rolled back completely, and
    /// `SET CONSTRAINTS ALL DEFERRED` only has an effect inside a transaction block.
    /// On error the `?` drops `tx`, which rolls it back.
    async fn execute_on_one_connection(&self, statements: &[String]) -> Result<(), DbError> {
        let mut tx = self.pool.begin().await?;
        for statement in statements {
            for single_sql in split_sql_statements(statement) {
                sqlx::query(AssertSqlSafe(single_sql))
                    .execute(&mut *tx)
                    .await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }

    async fn execute_params(&self, sql: &str, params: &[&str]) -> Result<(), DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }
        query.execute(&self.pool).await?;
        Ok(())
    }

    /// Not wrapped in a transaction on purpose: some statements
    /// (e.g. CREATE DATABASE / DROP DATABASE) cannot run inside one.
    async fn execute(&self, sql: &str) -> Result<(), DbError> {
        for single_sql in split_sql_statements(sql) {
            sqlx::query(AssertSqlSafe(single_sql))
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    async fn fetch_strings(&self, sql: &str) -> Result<Vec<String>, DbError> {
        let rows = sqlx::query(AssertSqlSafe(sql))
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| row.get::<String, _>(0))
            .collect())
    }

    async fn fetch_numbers(&self, sql: &str) -> Result<Vec<i64>, DbError> {
        let rows = sqlx::query(AssertSqlSafe(sql))
            .fetch_all(&self.pool)
            .await?;

        // MAX() on an empty table returns a single NULL row, so decode as Option
        let mut numbers = Vec::with_capacity(rows.len());
        for row in rows {
            if let Some(n) = row.try_get::<Option<i64>, _>(0)? {
                numbers.push(n);
            }
        }
        Ok(numbers)
    }

    async fn fetch_strings_params(
        &self,
        sql: &str,
        params: &[&str],
    ) -> Result<Vec<String>, DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        let rows = query.fetch_all(&self.pool).await?;

        Ok(rows
            .into_iter()
            .map(|row| row.get::<String, _>(0))
            .collect())
    }

    async fn fetch_count_params(&self, sql: &str, params: &[&str]) -> Result<i64, DbError> {
        let mut query = sqlx::query(AssertSqlSafe(sql));

        for param in params {
            query = query.bind(*param);
        }

        let row = query.fetch_one(&self.pool).await?;

        // COUNT(*) is BIGINT in PostgreSQL, so it decodes into i64 directly
        let count: i64 = row.try_get(0)?;

        Ok(count)
    }
}

/// Creates a database client using the engine and connection settings in `CONFIG`.
///
/// Connects to the configured MySQL, PostgreSQL, or SQLite database and returns
/// the client behind a [`DatabaseClient`] trait object.
///
/// # Errors
///
/// Returns [`DbError`] if the connection pool cannot be established. The
/// connection error is logged before it is returned.
pub async fn create_db_client() -> Result<Box<dyn DatabaseClient>, DbError> {
    match CONFIG.database.connection {
        DatabaseEngine::Mysql => {
            let con_string = format!(
                "mysql://{}:{}@{}:{}/{}",
                CONFIG.database.username,
                CONFIG.database.password,
                CONFIG.database.host,
                CONFIG.database.port,
                CONFIG.database.database,
            );

            let pool = MySqlPool::connect(&con_string).await.map_err(|e| {
                logger::error(&format!("MySQL connection error: {}", e));
                e
            })?;
            Ok(Box::new(MySqlClient { pool }))
        }
        DatabaseEngine::PgSql => {
            //let con_string = format!(
            //     "postgres://{}:{}@{}:{}/{}",
            //     CONFIG.database.username,
            //     CONFIG.database.password,
            //     CONFIG.database.host,
            //     CONFIG.database.port,
            //     CONFIG.database.database,
            // );

            let options = PgConnectOptions::new()
                .username(&CONFIG.database.username)
                .password(&CONFIG.database.password)
                .host(&CONFIG.database.host)
                .port(CONFIG.database.port)
                .database(&CONFIG.database.database);

            let pool = PgPool::connect_with(options).await.map_err(|e| {
                logger::error(&format!("Postgres connection error: {}", e));
                e
            })?;
            Ok(Box::new(PostgresClient { pool }))
        }

        DatabaseEngine::Sqlite => {
            let con_string = format!("sqlite://{}", CONFIG.database.database);

            let pool = SqlitePool::connect(&con_string).await.map_err(|e| {
                logger::error(&format!("SQLite connection error: {}", e));
                e
            })?;
            Ok(Box::new(SqliteClient { pool }))
        }
    }
}

pub async fn create_db_pool() -> Result<AnyPool, DbError> {
    install_default_drivers();

    let connection_string = match CONFIG.database.connection {
        DatabaseEngine::Mysql => {
            format!(
                "mysql://{}:{}@{}:{}/{}",
                CONFIG.database.username,
                CONFIG.database.password,
                CONFIG.database.host,
                CONFIG.database.port,
                CONFIG.database.database,
            )
        }

        DatabaseEngine::PgSql => {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                CONFIG.database.username,
                CONFIG.database.password,
                CONFIG.database.host,
                CONFIG.database.port,
                CONFIG.database.database,
            )
        }

        DatabaseEngine::Sqlite => {
            format!("sqlite://{}", CONFIG.database.database)
        }
    };

    let pool = AnyPool::connect(&connection_string).await.map_err(|e| {
        logger::error(&format!("Database connection error: {}", e));
        e
    })?;

    Ok(pool)
}
