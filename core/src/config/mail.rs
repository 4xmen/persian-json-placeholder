use serde::Serialize;
use std::{env, str::FromStr};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MailDriver {
    Smtp,
    SendMail,
    Log,
    Array,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MailEncryptType {
    Tls,
    Ssl,
    None,
}
#[derive(Debug, Clone, Serialize)]
pub struct MailConfig {
    pub mailer: MailDriver,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub encryption: MailEncryptType,
    pub from_address: String,
    pub from_name: String,
}

impl Default for MailConfig {
    fn default() -> Self {
        Self {
            mailer: MailDriver::SendMail,
            host: "smtp.example.com".into(),
            port: 587,
            username: "hello@example.com".into(),
            password: "secret".into(),
            encryption: MailEncryptType::Tls,
            from_address: "smtp.example.com".into(),
            from_name: "rustavel".into(),
        }
    }
}

impl MailConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self::default();

        if let Ok(v) = env::var("MAIL_MAILER") {
            cfg.mailer = v.parse().unwrap_or_else(|e| panic!("MAIL_MAILER: {e}"));
        }

        if let Ok(v) = env::var("MAIL_HOST") {
            cfg.host = v;
        }

        if let Ok(v) = env::var("MAIL_PORT") {
            cfg.port = v.parse().expect("MAIL_PORT must be a number");
        }

        if let Ok(v) = env::var("MAIL_USERNAME") {
            cfg.username = v;
        }

        if let Ok(v) = env::var("MAIL_PASSWORD") {
            cfg.password = v;
        }

        if let Ok(v) = env::var("MAIL_ENCRYPTION") {
            cfg.encryption = v.parse().unwrap_or_else(|e| panic!("MAIL_ENCRYPTION: {e}"));
        }

        if let Ok(v) = env::var("MAIL_FROM_ADDRESS") {
            cfg.from_address = v;
        }

        if let Ok(v) = env::var("MAIL_FROM_NAME") {
            cfg.from_name = v;
        }
        cfg
    }
}

// Using the standard `FromStr` trait gives us `.parse::<MailDriver>()`
// for free and avoids name clashes with an inherent `from_str` method.
impl FromStr for MailDriver {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "smtp" => Ok(Self::Smtp),
            "sendmail" => Ok(Self::SendMail),
            "log" => Ok(Self::Log),
            "array" => Ok(Self::Array),
            other => Err(format!(
                "unknown mail driver `{other}` (expected: smtp, sendmail, log, array)"
            )),
        }
    }
}

impl FromStr for MailEncryptType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "tls" => Ok(Self::Tls),
            "ssl" => Ok(Self::Ssl),
            // Disabling encryption must be explicit, never a fallback.
            "none" | "" => Ok(Self::None),
            other => Err(format!(
                "unknown mail encryption `{other}` (expected: tls, ssl, none)"
            )),
        }
    }
}
