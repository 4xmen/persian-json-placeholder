use crate::helper::database::{is_mysql, is_pgsql, is_sqlite};
use crate::sql::database_client::{DbError, create_db_pool_any, create_db_pool_mysql};
use std::any::TypeId;
use std::{env, str::FromStr};

// define default db pool
pub type DbPoolType = sqlx::MySqlPool;
// define default pool creator function
pub async fn create_db_pool() -> Result<DbPoolType, DbError> {
    create_db_pool_mysql().await
}

#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseEngine {
    Mysql,
    Sqlite,
    PgSql,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub connection: DatabaseEngine,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub prefix: String,
    pub collection: String,
    pub charset: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            connection: DatabaseEngine::Mysql,
            host: "127.0.0.1".into(),
            port: 3306,
            username: "root".into(),
            password: "".into(),
            database: "laravel".into(),
            prefix: "".into(),
            collection: "utf8mb4_unicode_ci".into(),
            charset: "utf8mb4".into(),
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();

        if let Ok(v) = env::var("DB_CONNECTION") {
            cfg.connection = v.parse().unwrap_or_else(|e| panic!("DB_CONNECTION: {e}"));
        }

        if let Ok(v) = env::var("DB_HOST") {
            cfg.host = v;
        }

        if let Ok(v) = env::var("DB_PORT") {
            cfg.port = v.parse().expect("DB_PORT must be a number");
        }

        if let Ok(v) = env::var("DB_USERNAME") {
            cfg.username = v;
        }

        if let Ok(v) = env::var("DB_PASSWORD") {
            cfg.password = v;
        }

        if let Ok(v) = env::var("DB_DATABASE") {
            cfg.database = v;
        }

        if let Ok(v) = env::var("DB_PREFIX") {
            cfg.prefix = v;
        }
        if let Ok(v) = env::var("DB_COLLATION") {
            cfg.collection = v;
        }
        if let Ok(v) = env::var("DB_CHARSET") {
            cfg.charset = v;
        }

        cfg
    }
}

impl FromStr for DatabaseEngine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "mysql" => Ok(Self::Mysql),
            "sqlite" => Ok(Self::Sqlite),
            "pgsql" => Ok(Self::PgSql),
            // Disabling encryption must be explicit, never a fallback.
            other => Err(format!(
                "unknown database driver `{other}` (expected: sqlite, mysql, pgsql)"
            )),
        }
    }
}

pub fn db_kind() -> &'static str {
    let tid = TypeId::of::<DbPoolType>();
    if tid == TypeId::of::<sqlx::PgPool>() {
        if !is_pgsql() {
            panic!("If you need use PgPool native you fix your config too.")
        }
        "postgres"
    } else if tid == TypeId::of::<sqlx::MySqlPool>() {
        if !is_mysql() {
            panic!("If you need use MySqlPool native you fix your config too.")
        }
        "mysql"
    } else if tid == TypeId::of::<sqlx::SqlitePool>() {
        if !is_sqlite() {
            panic!("If you need use SqlitePool native you fix your config too.")
        }
        "sqlite"
    } else if tid == TypeId::of::<sqlx::AnyPool>() {
        "any"
    } else {
        panic!("Unknown database type Look at: core/src/config/database.rs -> DbPoolType");
    }
}
