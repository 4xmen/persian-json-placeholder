use serde::Serialize;
use serde_json::Value;
use std::sync::LazyLock;

pub mod app;
pub mod database;

pub mod env;
pub mod http;
pub mod mail;
pub mod session;

#[derive(Debug, Serialize)]
pub struct Config {
    pub app: app::AppConfig,
    pub database: database::DatabaseConfig,
    pub mail: mail::MailConfig,
    pub http: http::HttpConfig,
    pub session: session::SessionConfig,
    // pub cache: CacheConfig, [sample how to add another one next time]
    #[serde(skip)]
    lookup: Value,
}

impl Config {
    fn load() -> Self {
        dotenv::dotenv().ok();

        let mut config = Self {
            app: app::AppConfig::from_env(),
            database: database::DatabaseConfig::from_env(),
            mail: mail::MailConfig::from_env(),
            http: http::HttpConfig::from_env(),
            session: session::SessionConfig::from_env(),
            lookup: Value::Null,
        };

        config.lookup = serde_json::to_value(&config).expect("failed to serialize config");

        config
    }

    pub fn get(&self, path: &str) -> Option<&Value> {
        path.split('.')
            .try_fold(&self.lookup, |value, key| value.get(key))
    }

    pub fn get_as<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let value = self
            .get(path)
            .ok_or_else(|| format!("config key not found: {path}"))?;

        serde_json::from_value(value.clone())
            .map_err(|err| format!("invalid value for {path}: {err}"))
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::load);
