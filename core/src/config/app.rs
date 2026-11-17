use crate::config::env::env_bool;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub env: String,
    pub debug: bool,
    pub host: String,
    pub port: u16,
    pub timezone: String,
    pub key: String,
    pub locale: String,
    pub fallback_locale: String,
    pub faker_locale: String,
    pub url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "Rustavel".into(),
            env: "production".into(),
            debug: false,
            host: "127.0.0.1".into(),
            port: 3000,
            timezone: "UTC".into(),
            key: "".into(),
            locale: "en".into(),
            fallback_locale: "en".into(),
            faker_locale: "en".into(),
            url: "http://localhost:3000/".into(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();

        if let Ok(v) = env::var("APP_NAME") {
            cfg.name = v;
        }

        if let Ok(v) = env::var("APP_ENV") {
            cfg.env = v;
        }

        if let Ok(v) = env::var("APP_DEBUG") {
            cfg.debug = env_bool(&v, true);
        }

        if let Ok(v) = env::var("APP_IP") {
            cfg.host = v;
        }

        if let Ok(v) = env::var("APP_PORT") {
            cfg.port = v.parse().expect("APP_PORT must be a number");
        }

        if let Ok(v) = env::var("APP_TIMEZONE") {
            cfg.timezone = v;
        }

        if let Ok(v) = env::var("APP_KEY") {
            cfg.key = v;
        }

        if let Ok(v) = env::var("APP_LOCALE") {
            cfg.locale = v;
        }

        if let Ok(v) = env::var("APP_FALLBACK_LOCALE") {
            cfg.fallback_locale = v;
        }

        if let Ok(v) = env::var("APP_FAKER_LOCALE") {
            cfg.faker_locale = v;
        }
        if let Ok(v) = env::var("APP_URL") {
            cfg.url = v;
        }

        cfg
    }
}
