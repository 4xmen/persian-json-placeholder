use std::sync::LazyLock;

pub mod app;
pub mod database;

pub mod env;
pub mod http;
pub mod mail;
pub mod session;

#[derive(Debug)]
pub struct Config {
    pub app: app::AppConfig,
    pub database: database::DatabaseConfig,
    pub mail: mail::MailConfig,
    pub http: http::HttpConfig,
    pub session: session::SessionConfig,
    // pub cache: CacheConfig, [sample how to add another one next time]
}

impl Config {
    fn load() -> Self {
        dotenv::dotenv().ok();

        Self {
            app: app::AppConfig::from_env(),
            database: database::DatabaseConfig::from_env(),
            mail: mail::MailConfig::from_env(),
            http: http::HttpConfig::from_env(),
            session: session::SessionConfig::from_env(),
            // cache: CacheConfig::from_env(), [sample how to add another one next time]
        }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);
