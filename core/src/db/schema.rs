use crate::config::CONFIG;
use crate::config::database::DatabaseEngine;
use crate::db::table::{ColumnDataType, Table, TableAction};
use crate::helper::database::{is_mysql, is_sqlite};
use crate::helper::terminal_ui::{Status, operation};
use crate::logger;
use crate::sql::database_client::{
    DatabaseClient, DbError, MySqlClient, PostgresClient, SqliteClient,
};
use crate::sql::generator::SqlGenerator;
use crate::sql::mysql::MySqlGenerator;
use crate::sql::postgres::PostgresGenerator;
use crate::sql::sqlite::SqliteGenerator;
use futures::future::join_all;
use illuminate_str::Str;
use sqlx::postgres::PgConnectOptions;
use sqlx::{MySqlPool, PgPool, SqlitePool};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tokio::time::Instant;

static TABLES: OnceCell<Vec<String>> = OnceCell::const_new();
static COLUMNS: OnceCell<Vec<String>> = OnceCell::const_new();
static TABLE_DOT_COLUMN: OnceCell<Arc<Vec<String>>> = OnceCell::const_new();

#[derive(Debug)]
pub struct ModelData {
    pub table: String,
    pub columns: String,
    pub primary_key: String,
    pub model: String,
}

pub async fn get_tables() -> &'static Vec<String> {
    TABLES
        .get_or_init(|| async {
            // println!("get_tables");
            match Schema::new().await {
                Ok(schema) => {
                    // If fetching tables fails, return empty vector instead of panicking
                    schema.get_tables().await.unwrap_or_default()
                }
                Err(e) => {
                    // Log the error and return empty vector
                    eprintln!("Failed to initialize schema: {}", e);
                    Vec::new()
                }
            }
        })
        .await
}

pub async fn get_columns() -> &'static Vec<String> {
    COLUMNS
        .get_or_init(|| async {
            // println!("get_columns");
            let schema = match Schema::new().await {
                Ok(schema) => schema,
                Err(e) => {
                    eprintln!("Failed to initialize schema: {}", e);
                    return Vec::new();
                }
            };

            // Fetch tables safely
            let tables = schema.get_tables().await.unwrap_or_default();

            let mut result = Vec::new();

            for table in tables {
                match schema.get_column_listing(&table).await {
                    Ok(columns) => {
                        for column in columns {
                            result.push(format!("{}.{}", table, column));
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch columns for table {}: {:?}", table, e);
                    }
                }
            }

            result
        })
        .await
}

#[derive(Debug)]
pub struct Schema {
    prefix: String,
    generator: Box<dyn SqlGenerator + Send + Sync>,
    pub client: Box<dyn DatabaseClient + Send + Sync>,
    debug: bool,
    tables: HashMap<String, Table>,
    // current: Option<Table>,
    pending: Vec<Table>,
}

impl Schema {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let debug = CONFIG.app.debug;

        let (generator, client): (Box<dyn SqlGenerator>, Box<dyn DatabaseClient>) =
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

                    (Box::new(MySqlGenerator), Box::new(MySqlClient { pool }))
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

                    (
                        Box::new(PostgresGenerator),
                        Box::new(PostgresClient { pool }),
                    )
                }

                DatabaseEngine::Sqlite => {
                    let con_string = format!("sqlite://{}", CONFIG.database.database);

                    let pool = SqlitePool::connect(&con_string).await.map_err(|e| {
                        logger::error(&format!("SQLite connection error: {}", e));
                        e
                    })?;

                    (Box::new(SqliteGenerator), Box::new(SqliteClient { pool }))
                }
            };

        Ok(Self {
            prefix: CONFIG.database.prefix.clone(),
            generator,
            client,
            debug,
            tables: HashMap::new(),
            // current: None,
            pending: Vec::new(),
        })
    }

    fn fix_table_name(&self, table_name: &str) -> String {
        format!("{}{}", self.prefix, table_name)
    }

    /// Drops a table if it exists.
    ///
    /// This method:
    /// - Normalizes the provided table_name using self.fix_table_name.
    /// - Generates a DROP TABLE IF EXISTS SQL statement via self.generator.
    /// - Executes the statement using self.client.
    ///
    /// # Behavior
    /// - Returns true if the SQL executes successfully.
    /// - Returns false on error. If self.debug is true, the error is logged into log file.
    ///
    /// # Parameters
    /// - table_name: Any type convertible to String representing the table name.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     s.drop_table_if_exists("test_table_here").await;
    /// }
    /// ```
    ///
    /// # Notes
    /// - The function performs no further validation of table_name.
    /// - Errors are swallowed (converted to false) after optional logging.
    pub async fn drop_table_if_exists(&self, table_name: impl Into<String>) -> bool {
        let sql = self
            .generator
            .drop_table_if_exists(&self.fix_table_name(&table_name.into()));
        match self.client.execute(&sql).await {
            Ok(_) => true,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                false
            }
        }
    }

    /// Drops a table if it exists with raw name without prefix
    ///
    /// This method:
    /// - Use name
    /// - Generates a DROP TABLE IF EXISTS SQL statement via self.generator.
    /// - Executes the statement using self.client.
    ///
    /// # Behavior
    /// - Returns true if the SQL executes successfully.
    /// - Returns false on error. If self.debug is true, the error is logged into log file.
    ///
    /// # Parameters
    /// - table_name: Any type convertible to String representing the table name.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     s.drop_table_if_exists("test_table_here").await;
    /// }
    /// ```
    ///
    /// # Notes
    /// - The function performs no further validation of table_name.
    /// - Errors are swallowed (converted to false) after optional logging.
    pub async fn drop_table_raw_name_if_exists(&self, table_name: impl Into<String>) -> bool {
        let sql = self.generator.drop_table_if_exists(&table_name.into());
        match self.client.execute(&sql).await {
            Ok(_) => true,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                false
            }
        }
    }

    /// Retrieves a list of all tables in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL query for listing tables.
    /// - Executes the query using the database client.
    /// - Converts the result into a vector of table names.
    ///
    /// # Behavior
    /// - Returns a `Result` containing a vector of table names on success.
    /// - Returns a `DbError` if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)`: A vector of table names if the query is successful.
    /// - `Err(DbError)`: An error if the table retrieval fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.get_tables().await {
    ///         Ok(tables) => println!("Tables: {:?}", tables),
    ///         Err(e) => eprintln!("Error retrieving tables: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - The exact list of tables depends on the database system and user permissions.
    /// - Performance may vary for databases with a large number of tables.
    pub async fn get_tables(&self) -> Result<Vec<String>, DbError> {
        match self
            .client
            .fetch_strings(&self.generator.get_tables())
            .await
        {
            Ok(tables) => Ok(tables),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }
    /// Attempts to retrieve a list of tables in the database, returning an empty vector on failure.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL query for listing tables.
    /// - Executes the query using the database client.
    /// - Returns the list of tables or an empty vector if the query fails.
    ///
    /// # Behavior
    /// - Returns a vector of table names if the query is successful.
    /// - Returns an empty vector if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Returns
    /// - `Vec<String>`: A list of table names, or an empty vector if retrieval fails.
    ///
    /// # Examples
    /// ```
    ///  use rustavel_core::db::schema::Schema;
    ///
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     let tables = s.try_get_tables().await;
    ///     println!("{}", tables.len());
    /// }
    /// ```
    ///
    /// # Notes
    /// - Unlike `get_tables()`, this method silently handles errors.
    /// - Useful in scenarios where table retrieval is optional or non-critical.
    /// - An empty result does not necessarily indicate a serious database problem.
    pub async fn try_get_tables(&self) -> Vec<String> {
        match self
            .client
            .fetch_strings(&self.generator.get_tables())
            .await
        {
            Ok(tables) => tables,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Vec::new()
            }
        }
    }

    /// Retrieves the list of columns for a specified database table.
    ///
    /// This method:
    /// - Generates a SQL query to list columns for the given table name.
    /// - Executes the query using the database client.
    /// - Returns the column names or an error if the query fails.
    ///
    /// # Behavior
    /// - Returns a `Result` containing a vector of column names on success.
    /// - Returns a `DbError` if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to retrieve columns from.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)`: A vector of column names if successful.
    /// - `Err(DbError)`: An error if column retrieval fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.get_column_listing("users").await {
    ///         Ok(columns) => println!("Columns: {:?}", columns),
    ///         Err(e) => eprintln!("Error retrieving columns: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Returns an empty vector if the table exists but has no columns or not exists at all.
    /// - The exact columns depend on the database system and table structure.
    pub async fn get_column_listing(
        &self,
        table_name: impl Into<String>,
    ) -> Result<Vec<String>, DbError> {
        match self
            .client
            .fetch_strings(
                &self
                    .generator
                    .get_column_listing(&self.fix_table_name(&table_name.into())),
            )
            .await
        {
            // note if table not found we don't have error just empty vector
            Ok(tables) => Ok(tables),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Attempts to retrieve the list of columns for a specified database table.
    ///
    /// This method:
    /// - Generates a SQL query to list columns for the given table name.
    /// - Executes the query using the database client.
    /// - Returns the column names or an empty vector on failure.
    ///
    /// # Behavior
    /// - Returns a vector of column names if the query is successful.
    /// - Returns an empty vector if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to retrieve columns from.
    ///
    /// # Returns
    /// - `Vec<String>`: A list of column names, or an empty vector if retrieval fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     let columns = s.try_get_column_listing("users").await;
    ///     println!("Columns found: {}", columns.len());
    /// }
    /// ```
    ///
    /// # Notes
    /// - Silently handles errors, making it suitable for non-critical column retrieval.
    /// - An empty result does not necessarily indicate a serious database problem.
    pub async fn try_get_column_listing(&self, table_name: impl Into<String>) -> Vec<String> {
        match self
            .client
            .fetch_strings(
                &self
                    .generator
                    .get_column_listing(&self.fix_table_name(&table_name.into())),
            )
            .await
        {
            Ok(tables) => tables,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Vec::new()
            }
        }
    }

    /// Retrieves the list of views in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL query for listing views.
    /// - Executes the query using the database client.
    /// - Returns the list of view names or an error if the query fails.
    ///
    /// # Behavior
    /// - Returns a `Result` containing a vector of view names on success.
    /// - Returns a `DbError` if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)`: A vector of view names if the query is successful.
    /// - `Err(DbError)`: An error if view retrieval fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.get_views().await {
    ///         Ok(views) => println!("Views: {:?}", views),
    ///         Err(e) => eprintln!("Error retrieving views: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - The list of views depends on the database system and user permissions.
    /// - Performance may vary for databases with a large number of views.
    pub async fn get_views(&self) -> Result<Vec<String>, DbError> {
        match self.client.fetch_strings(&self.generator.get_views()).await {
            Ok(tables) => Ok(tables),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }
    /// Attempts to retrieve the list of views in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL query for listing views.
    /// - Executes the query using the database client.
    /// - Returns the list of view names or an empty vector if the query fails.
    ///
    /// # Behavior
    /// - Returns a vector of view names if the query is successful.
    /// - Returns an empty vector if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Returns
    /// - `Vec<String>`: A list of view names, or an empty vector if retrieval fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///     async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     let views = s.try_get_views().await;
    ///     println!("Views found: {}", views.len());
    /// }
    /// ```
    ///
    /// # Notes
    /// - Unlike `get_views()`, this method silently handles errors.
    /// - Useful in scenarios where view retrieval is optional or non-critical.
    /// - An empty result does not necessarily indicate a serious database problem.
    pub async fn try_get_views(&self) -> Vec<String> {
        match self.client.fetch_strings(&self.generator.get_views()).await {
            Ok(tables) => tables,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Vec::new()
            }
        }
    }

    /// WIP : may use on postgres db
    pub fn get_current_schema_name(&self) -> &str {
        &CONFIG.database.database
    }

    /// Retrieves the list of foreign keys for a specific table in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL query for listing foreign keys.
    /// - Fixes the table name to ensure proper formatting.
    /// - Executes the query using the database client.
    /// - Returns the list of foreign key names or an error if the query fails.
    ///
    /// # Behavior
    /// - Returns a `Result` containing a vector of foreign key names on success.
    /// - Returns a `DbError` if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to retrieve foreign keys for.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)`: A vector of foreign key names if the query is successful.
    /// - `Err(DbError)`: An error if foreign key retrieval fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.get_foreign_keys("users").await {
    ///         Ok(foreign_keys) => println!("Foreign Keys: {:?}", foreign_keys),
    ///         Err(e) => eprintln!("Error retrieving foreign keys: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - The list of foreign keys depends on the database system and table structure.
    /// - Performance may vary for tables with a large number of foreign keys.
    pub async fn get_foreign_keys(
        &self,
        table_name: impl Into<String>,
    ) -> Result<Vec<String>, DbError> {
        match self
            .client
            .fetch_strings(
                &self
                    .generator
                    .get_foreign_keys(&self.fix_table_name(&table_name.into())),
            )
            .await
        {
            Ok(tables) => Ok(tables),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Attempts to retrieve the list of foreign keys for a specific table in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL query for listing foreign keys.
    /// - Fixes the table name to ensure proper formatting.
    /// - Executes the query using the database client.
    /// - Returns the list of foreign key names or an empty vector if the query fails.
    ///
    /// # Behavior
    /// - Returns a vector of foreign key names if the query is successful.
    /// - Returns an empty vector if the query fails.
    /// - If `self.debug` is true, any errors will be logged.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to retrieve foreign keys for.
    ///
    /// # Returns
    /// - `Vec<String>`: A list of foreign key names, or an empty vector if retrieval fails.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     let foreign_keys = s.try_get_foreign_keys("users").await;
    ///     println!("Foreign keys found: {}", foreign_keys.len());
    /// }
    /// ```
    ///
    /// # Notes
    /// - Unlike `get_foreign_keys()`, this method silently handles errors.
    /// - Useful in scenarios where foreign key retrieval is optional or non-critical.
    /// - An empty result does not necessarily indicate a serious database problem.
    pub async fn try_get_foreign_keys(&self, table_name: impl Into<String>) -> Vec<String> {
        match self
            .client
            .fetch_strings(
                &self
                    .generator
                    .get_foreign_keys(&self.fix_table_name(&table_name.into())),
            )
            .await
        {
            Ok(tables) => tables,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Vec::new()
            }
        }
    }

    /// Drops a specific table from the database.
    ///
    /// This method:
    /// - Fixes the table name to ensure proper formatting.
    /// - Uses the database generator to create a SQL DROP TABLE command.
    /// - Executes the drop command using the database client.
    ///
    /// # Behavior
    /// - Attempts to remove the specified table from the database.
    /// - Returns success if the table is dropped or does not exist.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to be dropped.
    ///
    /// # Returns
    /// - `Ok(())`: Successful table drop operation.
    /// - `Err(DbError)`: Error encountered during table drop.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.drop_table("users").await {
    ///         Ok(_) => println!("Table dropped successfully"),
    ///         Err(e) => eprintln!("Error dropping table: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Be cautious when dropping tables as this action is irreversible.
    /// - Requires appropriate database permissions.
    pub async fn drop_table(&self, table_name: impl Into<String>) -> Result<(), DbError> {
        self.client
            .execute(
                &self
                    .generator
                    .drop_table(&self.fix_table_name(&table_name.into())),
            )
            .await
            .map_err(|e| {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                e
            })
    }

    /// Drops a specific view from the database.
    ///
    /// This method:
    /// - Ensures the view name is properly formatted.
    /// - Utilizes the database generator to create a SQL DROP VIEW command.
    /// - Executes the drop command using the database client.
    ///
    /// # Behavior
    /// - Attempts to remove the specified view from the database.
    /// - Returns success if the view is dropped or does not exist.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `view_name`: The name of the view to be dropped.
    ///
    /// # Returns
    /// - `Ok(())`: Successful view drop operation.
    /// - `Err(DbError)`: Error encountered during view drop.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run() {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.drop_view("adult_users").await {
    ///         Ok(_) => println!("View dropped successfully"),
    ///         Err(e) => eprintln!("Error dropping view: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Be cautious when dropping views as this action is irreversible.
    /// - Requires appropriate database permissions.
    pub async fn drop_view(&self, view_name: impl Into<String>) -> Result<(), DbError> {
        self.client
            .execute(
                &self
                    .generator
                    .drop_view(&self.fix_table_name(&view_name.into())),
            )
            .await
            .map_err(|e| {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                e
            })
    }
    /// Drops all tables in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL command to drop all tables.
    /// - Executes the drop all tables command using the database client.
    ///
    /// # Behavior
    /// - Attempts to remove all tables from the database.
    /// - Returns success if all tables are dropped or no tables exist.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Returns
    /// - `Ok(())`: Successful drop of all tables.
    /// - `Err(DbError)`: Error encountered during table drop operation.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.drop_all_tables().await {
    ///         Ok(_) => println!("All tables dropped successfully"),
    ///         Err(e) => eprintln!("Error dropping all tables: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Extremely destructive operation - use with extreme caution.
    /// - Requires highest level of database permissions.
    /// - Permanently removes all data in all tables.
    /// - sqlx don't let us use more than one query so we have to drop tables one by one
    pub async fn drop_all_tables(&self) -> Result<(), DbError> {
        // FOREIGN_KEY_CHECKS / PRAGMA foreign_keys only affect the connection
        // they run on, so all statements must share one connection.
        let mut statements = vec![self.generator.disable_foreign_key_constraints()];
        for table in self.get_tables().await? {
            // names from get_tables() already carry the prefix
            statements.push(self.generator.drop_table(&table));
        }
        statements.push(self.generator.enable_foreign_key_constraints());
        self.client.execute_on_one_connection(&statements).await
    }

    /// Drops all views in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL command to drop all views.
    /// - Executes the drop all views command using the database client.
    ///
    /// # Behavior
    /// - Attempts to remove all views from the database.
    /// - Returns success if all views are dropped or no views exist.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Returns
    /// - `Ok(())`: Successful drop of all views.
    /// - `Err(DbError)`: Error encountered during view drop operation.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.drop_all_views().await {
    ///         Ok(_) => println!("All views dropped successfully"),
    ///         Err(e) => eprintln!("Error dropping all views: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Permanently removes all database views.
    /// - Requires appropriate database permissions.
    pub async fn drop_all_views(&self) -> Result<(), DbError> {
        let views = self.get_views().await?;
        let drop_futures: Vec<_> = views.into_iter().map(|view| self.drop_view(view)).collect();

        // execute all each other
        let results = join_all(drop_futures).await;

        // check result
        for res in results {
            res?; // خطاها propagate می‌شوند
        }
        Ok(())
    }

    /// Checks if a specific column exists in a given table.
    ///
    /// This method:
    /// - Fixes the table name to ensure proper formatting.
    /// - Uses the database generator to create a SQL query to check column existence.
    /// - Executes the query using the database client.
    ///
    /// # Behavior
    /// - Determines whether the specified column exists in the table.
    /// - Returns a boolean indicating column presence.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to check.
    /// - `column_name`: The name of the column to search for.
    ///
    /// # Returns
    /// - `Ok(bool)`:
    ///   - `true` if the column exists in the table.
    ///   - `false` if the column does not exist.
    /// - `Err(DbError)`: Error encountered during column existence check.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.has_column("users", "email").await {
    ///         Ok(exists) => println!("Column exists: {}", exists),
    ///         Err(e) => eprintln!("Error checking column: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Useful for schema introspection and validation.
    /// - Performance may vary depending on the database system.
    pub async fn has_column(
        &self,
        table_name: impl Into<String>,
        column_name: impl Into<String>,
    ) -> Result<bool, DbError> {
        let sql = self.generator.has_column(
            &self.fix_table_name(&table_name.into()),
            &column_name.into(),
        );
        match self.client.fetch_strings(&sql).await {
            Ok(handler) => Ok(!handler.is_empty()),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Checks if a specific table exists in the database.
    ///
    /// This method:
    /// - Fixes the table name to ensure proper formatting.
    /// - Uses the database generator to create a SQL query to check table existence.
    /// - Executes the query using the database client.
    ///
    /// # Behavior
    /// - Determines whether the specified table exists in the database.
    /// - Returns a boolean indicating table presence.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to check.
    ///
    /// # Returns
    /// - `Ok(bool)`:
    ///   - `true` if the table exists in the database.
    ///   - `false` if the table does not exist.
    /// - `Err(DbError)`: Error encountered during table existence check.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.has_table("users").await {
    ///         Ok(exists) => println!("Table exists: {}", exists),
    ///         Err(e) => eprintln!("Error checking table: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Useful for schema introspection and validation.
    /// - Performance may vary depending on the database system.
    pub async fn has_table(&self, table_name: impl Into<String>) -> Result<bool, DbError> {
        let sql = self
            .generator
            .has_table(&self.fix_table_name(&table_name.into()));
        match self.client.fetch_strings(&sql).await {
            Ok(handler) => Ok(!handler.is_empty()),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Checks if a specific view exists in the database.
    ///
    /// This method:
    /// - Fixes the view name to ensure proper formatting.
    /// - Uses the database generator to create a SQL query to check view existence.
    /// - Executes the query using the database client.
    ///
    /// # Behavior
    /// - Determines whether the specified view exists in the database.
    /// - Returns a boolean indicating view presence.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `table_name`: The name of the view to check.
    ///
    /// # Returns
    /// - `Ok(bool)`:
    ///   - `true` if the view exists in the database.
    ///   - `false` if the view does not exist.
    /// - `Err(DbError)`: Error encountered during view existence check.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.has_view("user_summary").await {
    ///         Ok(exists) => println!("View exists: {}", exists),
    ///         Err(e) => eprintln!("Error checking view: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Useful for schema introspection and validation.
    /// - Performance may vary depending on the database system.
    pub async fn has_view(&self, table_name: impl Into<String>) -> Result<bool, DbError> {
        let sql = self
            .generator
            .has_view(&self.fix_table_name(&table_name.into()));
        match self.client.fetch_strings(&sql).await {
            Ok(handler) => Ok(!handler.is_empty()),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Checks if a specific index exists on a table for given columns.
    ///
    /// This method:
    /// - Fixes the table name to ensure proper formatting.
    /// - Uses the database generator to create a SQL query to check index existence.
    /// - Executes the query using the database client.
    ///
    /// # Behavior
    /// - Determines whether an index exists on the specified table for given columns.
    /// - Returns a boolean indicating index presence.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to check for the index.
    /// - `columns_name`: A vector of column names that compose the index.
    ///
    /// # Returns
    /// - `Ok(bool)`:
    ///   - `true` if the index exists on the specified columns.
    ///   - `false` if the index does not exist.
    /// - `Err(DbError)`: Error encountered during index existence check.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.has_index("users", vec!["email", "username"]).await {
    ///         Ok(exists) => println!("Index exists: {}", exists),
    ///         Err(e) => eprintln!("Error checking index: {:?}", e),
    ///     }
    /// }
    ///
    pub async fn has_index(
        &self,
        table_name: impl Into<String>,
        columns_name: Vec<&str>,
    ) -> Result<bool, DbError> {
        let sql = self
            .generator
            .has_index(&self.fix_table_name(&table_name.into()), columns_name);
        match self.client.fetch_strings(&sql).await {
            Ok(handler) => Ok(!handler.is_empty()),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Creates a new database.
    ///
    /// # Behavior
    /// - Attempts to create a new database using the database generator.
    /// - Returns a boolean indicating success or failure.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `db_name`: The name of the database to create.
    ///
    /// # Returns
    /// - `true` if database creation was successful.
    /// - `false` if database creation failed.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let s = Schema::new().await.unwrap();
    ///     let created = s.create_database("my_new_database").await;
    ///     println!("Database created: {}", created);
    /// }
    /// ```
    ///
    /// # Notes
    /// - Not supported in SQLite, which uses file-based databases.
    /// - Behavior varies across different database systems.
    /// - Requires appropriate database permissions.
    pub async fn create_database(&self, db_name: impl Into<String>) -> bool {
        let sql = self.generator.create_database(&db_name.into());
        match self.client.execute(&sql).await {
            Ok(_) => true,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                false
            }
        }
    }

    /// Drops a database if it exists.
    ///
    /// # Behavior
    /// - Attempts to drop the specified database using the database generator.
    /// - Returns a boolean indicating success or failure.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `db_name`: The name of the database to drop.
    ///
    /// # Returns
    /// - `true` if database drop was successful or database did not exist.
    /// - `false` if database drop failed.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     let dropped = s.drop_database_if_exists("old_database").await;
    ///     println!("Database dropped: {}", dropped);
    /// }
    /// ```
    ///
    /// # Notes
    /// - Not supported in SQLite, which uses file-based databases.
    /// - Behavior varies across different database systems.
    /// - Requires highest level of database permissions.
    /// - Permanently removes the entire database and all its contents.
    pub async fn drop_database_if_exists(&self, db_name: impl Into<String>) -> bool {
        match self
            .client
            .execute(&self.generator.drop_database_if_exists(&db_name.into()))
            .await
        {
            Ok(_) => true,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                false
            }
        }
    }

    /// Disables foreign key constraints for the specified database.
    ///
    /// # Behavior
    /// - Attempts to disable foreign key constraints using the database generator.
    /// - Returns a boolean indicating success or failure.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `db_name`: The name of the database to disable constraints for.
    ///
    /// # Returns
    /// - `true` if disabling foreign key constraints was successful.
    /// - `false` if the operation failed.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     let disabled = s.disable_foreign_key_constraints().await;
    ///     println!("Foreign key constraints disabled: {}", disabled);
    /// }
    /// ```
    ///
    /// # Notes
    /// - Not supported in all database systems.
    /// - Useful for bulk data operations or migrations where foreign key checks need to be temporarily suspended.
    /// - Requires appropriate database permissions.
    pub async fn disable_foreign_key_constraints(&self) -> bool {
        match self
            .client
            .execute(&self.generator.disable_foreign_key_constraints())
            .await
        {
            Ok(_) => true,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                false
            }
        }
    }

    /// Enables foreign key constraints for the specified database.
    ///
    /// # Behavior
    /// - Attempts to enable foreign key constraints using the database generator.
    /// - Returns a boolean indicating success or failure.
    /// - Logs any errors if debug mode is enabled.
    ///
    /// # Parameters
    /// - `db_name`: The name of the database to enable constraints for.
    ///
    /// # Returns
    /// - `true` if enabling foreign key constraints was successful.
    /// - `false` if the operation failed.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     let enabled = s.enable_foreign_key_constraints().await;
    ///     println!("Foreign key constraints enabled: {}", enabled);
    /// }
    /// ```
    ///
    /// # Notes
    /// - Not supported in all database systems.
    /// - Typically used after performing operations that required foreign key constraints to be disabled.
    /// - Requires appropriate database permissions.
    pub async fn enable_foreign_key_constraints(&self) -> bool {
        match self
            .client
            .execute(&self.generator.enable_foreign_key_constraints())
            .await
        {
            Ok(_) => true,
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                false
            }
        }
    }

    /// Renames a table in the database.
    ///
    /// This method:
    /// - Uses the database generator to create a SQL RENAME TABLE command.
    /// - Executes the rename operation using the database client.
    ///
    /// # Behavior
    /// - Attempts to rename a table from the old name to the new name.
    /// - Logs any errors if debug mode is enabled.
    /// - Returns success if the table is successfully renamed.
    ///
    /// # Parameters
    /// - `old_table_name`: The current name of the table.
    /// - `new_table_name`: The desired new name for the table.
    ///
    /// # Returns
    /// - `Ok(())`: Successful table renaming.
    /// - `Err(DbError)`: Error encountered during table renaming.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.rename("old_users", "new_users").await {
    ///         Ok(_) => println!("Table renamed successfully"),
    ///         Err(e) => eprintln!("Error renaming table: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Behavior may vary across different database systems.
    /// - Requires appropriate database permissions.
    /// - Renaming a table can potentially break existing references or queries.
    pub async fn rename(
        &self,
        old_table_name: impl Into<String>,
        new_table_name: impl Into<String>,
    ) -> Result<(), DbError> {
        let sql = self
            .generator
            .rename(&old_table_name.into(), &new_table_name.into());
        match self.client.execute(&sql).await {
            Ok(_) => Ok(()),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Renames all tables by replacing the current prefix with a new prefix.
    ///
    /// This method:
    /// - Retrieves the list of all tables in the database.
    /// - Renames each table by replacing the existing prefix with the new prefix.
    /// - Updates the internal prefix value after successful renaming.
    ///
    /// # Behavior
    /// - Attempts to rename all tables with the new prefix.
    /// - Logs a warning about updating the configuration file.
    /// - Returns success if all table renames are successful.
    /// - Returns an error if table retrieval or renaming fails.
    ///
    /// # Parameters
    /// - `new_prefix`: The new prefix to be applied to table names.
    ///
    /// # Returns
    /// - `Ok(())`: Successfully renamed all tables and updated prefix.
    /// - `Err(DbError)`: Error encountered during table renaming process.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// async fn run()  {
    ///     let mut s = Schema::new().await.unwrap();
    ///     match s.rename_prefix("new_").await {
    ///         Ok(_) => println!("All tables renamed successfully"),
    ///         Err(e) => eprintln!("Error renaming tables: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Requires manually updating the prefix in the configuration file too.
    /// - Potentially destructive operation - use with caution.
    /// - Affects all tables in the database with the current prefix.
    pub async fn rename_prefix(&mut self, new_prefix: &str) -> Result<(), DbError> {
        match self.get_tables().await {
            Ok(tables) => {
                for table in tables {
                    // self.rename(table.clone(), table.replace(&self.prefix, new_prefix))
                    //     .await?

                    // only the leading prefix; replace("") would insert between every char
                    let bare = table.strip_prefix(self.prefix.as_str()).unwrap_or(&table);
                    self.rename(table.clone(), format!("{}{}", new_prefix, bare))
                        .await?
                }
                self.prefix = new_prefix.to_string();
                logger::warn("After change prefix you need to fix your config prefix too `.env`");
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Defines and creates a new table in the schema.
    ///
    /// This method:
    /// - Initializes a new `Table` with a specified name.
    /// - Allows customized configuration of the table's columns and properties via a closure.
    /// - Checks if a table with the same name already exists, updating its state or inserting a new entry as appropriate.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to be created.
    /// - `f`: A closure that receives a mutable reference to a `Table` object, allowing configuration of the table's structure.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current schema builder instance, allowing for method chaining.
    ///
    /// # Behavior
    /// - If the table already exists, its state is updated only if the existing structure is the same as the new definition.
    /// - If the table does not exist, it is added to the schema's internal table representation.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let mut s = Schema::new().await.unwrap();
    ///     s.create("users", |table| {
    ///         table.id();
    ///         table.string("username", 255);
    ///         table.integer("age");
    ///     });
    ///
    ///     // Further operations...
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method enables flexible table definitions and can be called multiple times to define different tables.
    /// - Ensure appropriate database permissions are granted to create tables in the schema.
    pub fn create<F>(&mut self, table_name: impl Into<String>, f: F) -> &mut Self
    where
        F: FnOnce(&mut Table),
    {
        let name = table_name.into();
        let mut table = Table::new(&format!("{}{}", CONFIG.database.prefix, name));
        table.action = TableAction::Create;
        f(&mut table);

        // check if the table already exists
        if let Some(tbl) = self.tables.get_mut(&name) {
            // update table state only if the existing table is the same
            for column in &table.columns {
                tbl.columns.push(column.clone());
            }
        } else {
            // insert the new table if it doesn't exist
            self.tables.insert(name.clone(), table.clone());
        }

        // self.current = Some(table);
        self.pending.push(table);
        // println!("{:?}",self.tables.keys().cloned().collect::<Vec<_>>());

        self
    }

    /// Alters an existing table in the schema.
    ///
    /// This method:
    /// - Initializes a `Table` object representing the specified table name.
    /// - Allows for modification of the table's columns and properties via a closure.
    /// - Checks if the specified table already exists and updates its state accordingly.
    ///
    /// # Parameters
    /// - `table_name`: The name of the table to be altered.
    /// - `f`: A closure that receives a mutable reference to a `Table` object, allowing for the configuration of the table's structure.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current schema builder instance, allowing for method chaining.
    ///
    /// # Behavior
    /// - If the table already exists, its state is updated based on the alterations made in the provided closure.
    /// - If the table does not exist, it is added to the schema's internal representation as a new entry.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let mut s = Schema::new().await.unwrap();
    ///     s.table("users", |table| {
    ///         table.string("email", 255).unique(); // Adding a new column
    ///     });
    ///
    ///     // Further operations...
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method enables modifications to the existing table structure and can be invoked multiple times for different tables.
    /// - Ensure appropriate database permissions are granted to alter tables in the schema.
    pub fn table<F>(&mut self, table_name: impl Into<String>, f: F) -> &mut Self
    where
        F: FnOnce(&mut Table),
    {
        let name = table_name.into();
        let mut table = Table::new(&format!("{}{}", CONFIG.database.prefix, name));
        table.action = TableAction::Alter;

        f(&mut table);

        // println!("{:?}",self.tables.keys().cloned().collect::<Vec<_>>());
        // check if the table already exists
        if let Some(tbl) = self.tables.get_mut(&name) {
            // update table state only if the existing table is the same
            for column in &table.columns {
                tbl.columns.push(column.clone());
            }
            // dbg!(&tbl.columns);
        } else {
            // insert the new table if it doesn't exist
            self.tables.insert(name.clone(), table.clone());
        }

        // self.current = Some(table);
        self.pending.push(table);
        self
    }

    /// Executes the last migration generated by the schema.
    ///
    /// This method:
    /// - Retrieves the last migration's data information.
    /// - Generates the SQL commands to apply the migration, including adding or dropping columns and creating foreign keys.
    /// - Executes the generated SQL command using the database client.
    ///
    /// # Behavior
    /// - Attempts to apply the last migration to the database.
    /// - Returns success if the migration is executed successfully.
    /// - Logs the operation status if debug mode is enabled.
    ///
    /// # Parameters
    /// - `final_name`: The name of the migration being executed.
    /// - `duration`: The time duration taken to execute the migration.
    ///
    /// # Returns
    /// - `Ok(())`: Successful execution of the migration.
    /// - `Err(DbError)`: Error encountered during migration execution.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// use std::time::Instant;
    ///
    /// async fn run() {
    ///     let mut s = Schema::new().await.unwrap();
    ///     let start_time = Instant::now();
    ///     s.create("mocks",|table|{
    ///         table.id();
    ///         table.string("name",110).nullable().comment("name of mock");
    ///     });
    ///     match s.execute_migration("add_user_table", &start_time.into()).await {
    ///         Ok(_) => println!("Migration executed successfully"),
    ///         Err(e) => eprintln!("Error executing migration: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - Ensure that the current migration data is valid before calling this method.
    /// - Requires appropriate database permissions to execute the migration.
    pub async fn execute_migration(
        &mut self,
        final_name: &str,
        duration: &Instant,
    ) -> Result<(), DbError> {
        if self.pending.is_empty() {
            return Err(DbError::InvalidTable);
        }
        // run every table defined since the last execute, in order
        let tables = std::mem::take(&mut self.pending);
        for table in &tables {
            let sql = self.table_to_sql(table);
            if let Err(e) = self.client.execute(&sql).await {
                operation(final_name, duration.elapsed(), Status::Failed);
                return Err(e);
            }
        }
        operation(final_name, duration.elapsed(), Status::Done);
        Ok(())
    }

    /// Drops table definitions that were collected but not executed,
    /// e.g. from a migration that has already run.
    pub fn clear_pending(&mut self) {
        self.pending.clear();
    }

    fn table_to_sql(&self, table: &Table) -> String {
        let mut body = vec![];
        let mut foot = vec![];
        // post sql are single sql
        let mut post = vec![];
        let mut spliter = String::new();
        for column in &table.columns {
            let (b, f, p, s) = self.generator.column(column, &table.name, &table.action);
            body.push(b);
            if !f.is_empty() {
                foot.push(f);
            }
            if !p.is_empty() {
                post.push(p);
            }
            spliter = s;
        }
        // for column in &table.drop_columns {
        //     body.push(self.generator.drop_column(&column));
        // }

        // rename table's cols
        for (old_name, new_name) in table.get_rename_columns() {
            if is_mysql() {
                body.push(
                    self.generator
                        .rename_column(&table.name, old_name, new_name),
                )
            } else {
                post.push(
                    self.generator
                        .rename_column(&table.name, old_name, new_name),
                )
            }
        }

        for column in table.get_drop_columns() {
            body.push(self.generator.drop_column(&table.name, column));
        }

        let drop_foreign_keys = table.get_drop_foreign_keys();
        if !drop_foreign_keys.is_empty() && is_sqlite() {
            logger::warn("SQLite does not support drop foreign keys");
        } else {
            for key in drop_foreign_keys {
                body.push(self.generator.drop_foreign_key(key));
            }
        }

        // for key in &table.foreign_keys {
        //     let str = self.generator.foreign_key(&key, &table.name, &table.action);

        for key in table.get_foreign_keys() {
            // the referenced table carries the same prefix as every other table
            let mut key = key.clone();
            key.foreign_table = self.fix_table_name(&key.foreign_table);
            let str = self.generator.foreign_key(&key, &table.name, &table.action);

            if !str.is_empty() {
                foot.push(str);
            }
        }
        body.append(&mut foot);
        // println!("{:?}",body);

        let sql = Str::implode(&spliter, body);
        self.generator
            .table_sql(&table.name, &sql, &Str::implode(";\n", post), &table.action)
    }

    /// Checks if the migrations repository exists in the database.
    ///
    /// This method:
    /// - Queries the database to determine if the `migrations` table is present.
    ///
    /// # Behavior
    /// - Returns `true` if the migrations repository exists, `false` otherwise.
    /// - Returns an error if there is an issue querying the database.
    ///
    /// # Returns
    /// - `Ok(true)`: The migrations repository exists.
    /// - `Ok(false)`: The migrations repository does not exist.
    /// - `Err(DbError)`: Error encountered while checking the repository's existence.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.repository_exists().await {
    ///         Ok(exists) if exists => println!("Migrations repository exists"),
    ///         Ok(_) => println!("Migrations repository does not exist"),
    ///         Err(e) => eprintln!("Error checking repository: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method relies on the `has_table` function to perform the check.
    /// - Requires appropriate database permissions to query the table existence.
    pub async fn repository_exists(&self) -> Result<bool, DbError> {
        self.has_table("migrations").await
    }

    /// Retrieves a list of successfully executed migrations from the database.
    ///
    /// This method:
    /// - Queries the `migrations` table to fetch the names of migrations that have been executed.
    ///
    /// # Behavior
    /// - Returns a vector of migration names that have been run.
    /// - If the migrations table does not exist, it returns an empty vector without an error.
    /// - Logs any errors encountered during the operation if debug mode is enabled.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)`: A vector containing the names of executed migrations.
    /// - `Err(DbError)`: Error encountered while fetching the migrations.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.get_ran_migrations().await {
    ///         Ok(ran) => println!("Ran migrations: {:?}", ran),
    ///         Err(e) => eprintln!("Error retrieving ran migrations: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method relies on the `fetch_strings` function to perform the retrieval.
    /// - Ensure appropriate database permissions are granted to access the migrations table.
    pub async fn get_ran_migrations(&self) -> Result<Vec<String>, DbError> {
        // match self.client.fetch_strings(&self.generator.get_ran()).await {
        let table = self.fix_table_name("migrations");
        match self
            .client
            .fetch_strings(&self.generator.get_ran(&table))
            .await
        {
            // note if table not found we don't have error just empty vector
            Ok(ran) => Ok(ran),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Retrieves a list of successfully executed migrations from the database  greater than batch.
    ///
    /// This method:
    /// - Queries the `migrations` table to fetch the names of migrations that have been executed.
    ///
    /// # Behavior
    /// - Returns a vector of migration names that have been run.
    /// - If the migrations table does not exist, it returns an empty vector without an error.
    /// - Logs any errors encountered during the operation if debug mode is enabled.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)`: A vector containing the names of executed migrations.
    /// - `Err(DbError)`: Error encountered while fetching the migrations.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.get_ran_migrations().await {
    ///         Ok(ran) => println!("Ran migrations: {:?}", ran),
    ///         Err(e) => eprintln!("Error retrieving ran migrations: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method relies on the `fetch_strings` function to perform the retrieval.
    /// - Ensure appropriate database permissions are granted to access the migrations table.
    pub async fn get_ran_migrations_gt(&self, batch: i64) -> Result<Vec<String>, DbError> {
        match self
            .client
            // .fetch_strings_params(&self.generator.get_ran_gt(), &[&batch.to_string()])
            .fetch_strings_params(
                &self
                    .generator
                    .get_ran_gt(&self.fix_table_name("migrations")),
                &[&batch.to_string()],
            )
            .await
        {
            // note if table not found we don't have error just empty vector
            Ok(ran) => Ok(ran),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Retrieves the next available batch number for migrations.
    ///
    /// This method:
    /// - Queries the database to find the highest batch number currently recorded.
    /// - Increments the highest number by one to determine the next batch number.
    ///
    /// # Behavior
    /// - Returns `1` if there are no existing batch numbers (i.e., the table is empty).
    /// - Logs any errors encountered during the operation if debug mode is enabled.
    ///
    /// # Returns
    /// - `Ok(i64)`: The next available batch number.
    /// - `Err(DbError)`: Error encountered while fetching the batch number.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let s = Schema::new().await.unwrap();
    ///     match s.get_next_batch_number().await {
    ///         Ok(next_batch) => println!("Next batch number: {}", next_batch),
    ///         Err(e) => eprintln!("Error retrieving next batch number: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method relies on the `fetch_numbers` function to perform the query.
    /// - Ensure appropriate database permissions are granted to access the required data.
    pub async fn get_next_batch_number(&self) -> Result<i64, DbError> {
        match self
            .client
            // .fetch_numbers(&self.generator.get_next_batch_number())
            .fetch_numbers(
                &self
                    .generator
                    .get_next_batch_number(&self.fix_table_name("migrations")),
            )
            .await
        {
            Ok(max_vec) => {
                // if table is empty
                if max_vec.is_empty() {
                    Ok(1)
                } else {
                    Ok(max_vec[0] + 1)
                }
            }
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Creates the migration table in the database.
    ///
    /// This method:
    /// - Initializes a new table named `migrations`.
    /// - Defines the structure of the table, including:
    ///   - An `id` column as the primary key.
    ///   - A `migration` column to store migration names (up to 255 characters).
    ///   - A `batch` column to track the batch number for the migration.
    ///
    /// # Behavior
    /// - Executes the creation of the table and logs the operation's duration.
    /// - Returns success if the table is created successfully.
    /// - Returns an error if any issue is encountered during the creation process.
    ///
    /// # Returns
    /// - `Ok(())`: Successful creation of the migration table.
    /// - `Err(DbError)`: Error encountered during table creation.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    /// use std::time::Instant;
    ///
    /// async fn run() {
    ///     let mut s = Schema::new().await.unwrap();
    ///     match s.create_migration_table().await {
    ///         Ok(_) => println!("Migration table created successfully"),
    ///         Err(e) => eprintln!("Error creating migration table: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - The method utilizes a closure to define the table schema and then executes the migration creation.
    /// - Ensure appropriate database permissions are granted to create tables.
    pub async fn create_migration_table(&mut self) -> Result<(), DbError> {
        let start = Instant::now();
        self.create("migrations", |table| {
            table.id();
            table.string("migration", 255);
            table.integer("batch");
        })
        .execute_migration("Creating migration table", &start)
        .await?;
        Ok(())
    }

    /// Adds a migrated table entry to the migrations repository.
    ///
    /// This method:
    /// - Inserts a new record into the `migrations` table with the specified migration name and batch number.
    ///
    /// # Behavior
    /// - Executes the insert operation using parameterized SQL for security against SQL injection.
    /// - Logs any errors encountered during the operation if debug mode is enabled.
    ///
    /// # Parameters
    /// - `migration_name`: The name of the migration being added.
    /// - `batch_number`: The batch number associated with the migration.
    ///
    /// # Returns
    /// - `Ok(())`: Successful addition of the migrated table entry.
    /// - `Err(DbError)`: Error encountered while adding the entry.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let mut s = Schema::new().await.unwrap();
    ///     match s.add_migrated_table("add_user_table", 1).await {
    ///         Ok(_) => println!("Migration added successfully"),
    ///         Err(e) => eprintln!("Error adding migration: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method utilizes a prepared statement to execute the insertion safely.
    /// - Ensure appropriate database permissions are granted to modify the migrations table.
    pub async fn add_migrated_table(
        &mut self,
        migration_name: &str,
        batch_number: i64,
    ) -> Result<(), DbError> {
        // let sql = &self.generator.add_migrated_table();
        let sql = &self
            .generator
            .add_migrated_table(&self.fix_table_name("migrations"));
        match self
            .client
            .execute_params(sql, &[migration_name, &batch_number.to_string()])
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    /// Removes a migrated table entry from the migrations repository.
    ///
    /// This method:
    /// - Deletes the specified migration entry from the `migrations` table based on the migration name.
    ///
    /// # Behavior
    /// - Executes the deletion using parameterized SQL for security against SQL injection.
    /// - Logs any errors encountered during the operation if debug mode is enabled.
    ///
    /// # Parameters
    /// - `migration_name`: The name of the migration to be removed.
    ///
    /// # Returns
    /// - `Ok(())`: Successful removal of the migrated table entry.
    /// - `Err(DbError)`: Error encountered while removing the entry.
    ///
    /// # Examples
    /// ```rust
    /// use rustavel_core::db::schema::Schema;
    ///
    /// async fn run() {
    ///     let mut s = Schema::new().await.unwrap();
    ///     match s.rem_migrated_table("add_user_table").await {
    ///         Ok(_) => println!("Migration removed successfully"),
    ///         Err(e) => eprintln!("Error removing migration: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// # Notes
    /// - This method utilizes a prepared statement for safe execution of the deletion.
    /// - Ensure appropriate database permissions are granted to modify the migrations table.
    pub async fn rem_migrated_table(&self, migration_name: &str) -> Result<(), DbError> {
        // let sql = &self.generator.rem_migrated_table();
        let sql = &self
            .generator
            .rem_migrated_table(&self.fix_table_name("migrations"));
        match self.client.execute_params(sql, &[migration_name]).await {
            Ok(_) => Ok(()),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                Err(e)
            }
        }
    }

    // pub async fn exists_record(&self, table: &str, column: &str, wanted: &str) -> bool {
    //     // ----------------------------------------------------
    //     // Perform identifier validation (table & column) using a cached schema whitelist.
    //     // Schema metadata is initialized once and reused from memory (OnceCell),
    //     // providing O(1)-like lookup performance under high concurrency.
    //     // This ensures query safety by preventing injection through dynamic identifiers
    //     // before building the final SQL statement.
    //     // ----------------------------------------------------
    //     let tables = get_tables().await;
    //     if !tables.contains(&table.to_string()) {
    //         logger::error(&format!("table not exists {}", table));
    //         logger::info("May you need to run migrations or restart app to re-cache tables!?");
    //         return false;
    //     }
    //     let columns = get_columns().await;
    //     if !columns.contains(&format!("{}.{}", table, column)) {
    //         logger::error(&format!("column `{}` not exists in {}", column, table));
    //         logger::info("May you need to run migrations or restart app to re-cache tables!?");
    //         return false;
    //     }
    //     // check exists record
    //     let sql = &self.generator.record_exists(table, column);
    //     match self.client.fetch_count_params(sql, &[wanted]).await {
    //         Ok(result) => {
    //             if result == 0 {
    //                 return false;
    //             }
    //             true
    //         }
    //         Err(e) => {
    //             if self.debug {
    //                 logger::error(&format!("{:?}", e));
    //             }
    //             false
    //         }
    //     }
    // }
    //
    // pub async fn exists_record_except(
    //     &self,
    //     table: &str,
    //     column: &str,
    //     wanted: &str,
    //     except_col: &str,
    //     except_val: &str,
    // ) -> bool {
    //     // ----------------------------------------------------
    //     // Perform identifier validation (table & column) using a cached schema whitelist.
    //     // Schema metadata is initialized once and reused from memory (OnceCell),
    //     // providing O(1)-like lookup performance under high concurrency.
    //     // This ensures query safety by preventing injection through dynamic identifiers
    //     // before building the final SQL statement.
    //     // ----------------------------------------------------
    //     let tables = get_tables().await;
    //     if !tables.contains(&table.to_string()) {
    //         logger::error(&format!("table not exists {}", table));
    //         logger::info("May you need to run migrations or restart app to re-cache tables!?");
    //         return false;
    //     }
    //     let columns = get_columns().await;
    //     if !columns.contains(&format!("{}.{}", table, column)) {
    //         logger::error(&format!("column `{}` not exists in {}", column, table));
    //         logger::info("May you need to run migrations or restart app to re-cache tables!?");
    //         return false;
    //     }
    //     if !columns.contains(&format!("{}.{}", table, except_col)) {
    //         logger::error(&format!("column `{}` not exists in {}", column, table));
    //         logger::info("May you need to run migrations or restart app to re-cache tables!?");
    //         return false;
    //     }
    //     // check exists record
    //     let sql = &self
    //         .generator
    //         .record_exists_except(table, column, except_col);
    //     match self
    //         .client
    //         .fetch_count_params(sql, &[wanted, except_val])
    //         .await
    //     {
    //         Ok(result) => {
    //             if result == 0 {
    //                 return false;
    //             }
    //             true
    //         }
    //         Err(e) => {
    //             if self.debug {
    //                 logger::error(&format!("{:?}", e));
    //             }
    //             false
    //         }
    //     }
    // }

    /// `Some(true/false)` if the lookup ran, `None` if it could not run
    /// (unknown table/column or a database error). Validators must treat
    /// `None` as a failure, otherwise `unique` would pass on any error.
    pub async fn exists_record(&self, table: &str, column: &str, wanted: &str) -> Option<bool> {
        // ----------------------------------------------------
        // Identifiers are checked against a cached schema whitelist
        // before being placed into the SQL statement.
        // ----------------------------------------------------
        let table = self.fix_table_name(table);

        let tables_columns = self.get_db_tables_columns_names();
        match tables_columns.await {
            Ok(columns) => {
                if !columns.contains(&format!("{}.{}", table, column)) {
                    logger::error(&format!(
                        "table or column `{}` not exists in {}",
                        column, table
                    ));
                    logger::info(
                        "May you need to run migrations or restart app to re-cache tables!?",
                    );
                    return None;
                }
            }
            Err(_e) => {
                return None;
            }
        }
        let sql = &self.generator.record_exists(&table, column);
        match self.client.fetch_count_params(sql, &[wanted]).await {
            Ok(count) => Some(count > 0),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                None
            }
        }
    }

    pub async fn exists_record_except(
        &self,
        table: &str,
        column: &str,
        wanted: &str,
        except_col: &str,
        except_val: &str,
    ) -> Option<bool> {
        // ----------------------------------------------------
        // Identifiers are checked against a cached schema whitelist
        // before being placed into the SQL statement.
        // ----------------------------------------------------
        let tables_columns = self.get_db_tables_columns_names();
        match tables_columns.await {
            Ok(columns) => {
                if !columns.contains(&format!("{}.{}", table, column)) {
                    logger::error(&format!(
                        "table or column `{}` not exists in {}",
                        column, table
                    ));
                    logger::info(
                        "May you need to run migrations or restart app to re-cache tables!?",
                    );
                    return None;
                }
            }
            Err(_e) => {
                return None;
            }
        }
        let sql = &self
            .generator
            .record_exists_except(table, column, except_col);
        match self
            .client
            .fetch_count_params(sql, &[wanted, except_val])
            .await
        {
            Ok(count) => Some(count > 0),
            Err(e) => {
                if self.debug {
                    logger::error(&format!("{:?}", e));
                }
                None
            }
        }
    }

    // pub fn final_tables(&self) -> &HashMap<String,Table> {
    //     &self.tables
    // }

    pub fn to_struct(&self) -> Vec<ModelData> {
        // let mut has_id = false;
        // self.tables
        //     .iter()
        //     .map(|(table_name, table)| {
        self.tables
            .iter()
            .map(|(table_name, table)| {
                // per table: one table's id must not leak into the next
                let mut has_id = false;
                let mut cols = Vec::with_capacity(table.columns.len() * 2);

                for column in &table.columns {
                    // check by datatype fix columns name
                    match column.data_type {
                        ColumnDataType::DTMorph => {
                            cols.push(format!("{}_type", column.name));
                            cols.push(format!("{}_id", column.name));
                        }
                        ColumnDataType::DTTimestamps => {
                            cols.push("created_at".to_owned());
                            cols.push("updated_at".to_owned());
                        }
                        _ => {
                            if ColumnDataType::DTId == column.data_type {
                                has_id = true;
                            }
                            cols.push(column.name.clone());
                        }
                    }
                }

                ModelData {
                    table: table_name.clone(),
                    columns: cols
                        .iter()
                        .map(|c| format!("\"{}\"", c))
                        .collect::<Vec<_>>()
                        .join(", "),
                    primary_key: if has_id {
                        "id".to_owned()
                    } else {
                        "".to_owned()
                    },
                    model: table.to_struct(),
                }
            })
            .collect()
    }

    async fn get_db_tables_columns_names(&self) -> Result<Arc<Vec<String>>, DbError> {
        let result = TABLE_DOT_COLUMN
            .get_or_try_init(|| async {
                let sql = self.generator.get_db_tables_columns_struct();
                self.client.fetch_strings(&sql).await.map(Arc::new)
            })
            .await?;

        Ok(Arc::clone(result))
    }
}
