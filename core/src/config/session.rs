use super::env::{env_bool, env_parse};
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SessionDriver {
    /// Process-local store. Fine for development; lost on restart and
    /// not shared between instances. A `Database` driver comes next.
    Memory,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum SameSite {
    Lax,
    Strict,
    None,
}

impl SameSite {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lax => "Lax",
            Self::Strict => "Strict",
            Self::None => "None",
        }
    }
}

impl FromStr for SessionDriver {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "memory" => Ok(Self::Memory),
            other => Err(format!(
                "unknown session driver `{other}` (expected: memory)"
            )),
        }
    }
}

impl FromStr for SameSite {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "lax" => Ok(Self::Lax),
            "strict" => Ok(Self::Strict),
            "none" => Ok(Self::None),
            other => Err(format!(
                "unknown SameSite `{other}` (expected: lax, strict, none)"
            )),
        }
    }
}

/// Session configuration (all values overridable from `.env`).
#[derive(Debug, Clone, Serialize)]
pub struct SessionConfig {
    /// SESSION_DRIVER
    pub driver: SessionDriver,
    /// SESSION_COOKIE. In production over HTTPS consider a `__Host-` prefix
    /// (e.g. `__Host-rustavel_session`): it forces Secure + Path=/ + no Domain.
    pub cookie_name: String,
    /// SESSION_LIFETIME_MINUTES
    pub lifetime_minutes: u64,
    /// SESSION_SECURE_COOKIE: send the cookie over HTTPS only.
    /// Defaults to false so local http development works; enable in production.
    pub secure: bool,
    /// SESSION_SAME_SITE: Lax is the best default (it blocks cross-site POSTs
    /// and still lets normal link navigation keep the user logged in).
    pub same_site: SameSite,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            driver: SessionDriver::Memory,
            cookie_name: "rustavel_session".into(),
            lifetime_minutes: 120,
            secure: false,
            same_site: SameSite::Lax,
        }
    }
}

impl SessionConfig {
    pub fn from_env() -> Self {
        let d = Self::default();
        let cfg = Self {
            driver: env_parse("SESSION_DRIVER", d.driver),
            cookie_name: env_parse("SESSION_COOKIE", d.cookie_name),
            lifetime_minutes: env_parse("SESSION_LIFETIME_MINUTES", d.lifetime_minutes),
            secure: env_bool("SESSION_SECURE_COOKIE", d.secure),
            same_site: env_parse("SESSION_SAME_SITE", d.same_site),
        };
        // Browsers reject `SameSite=None` cookies that are not `Secure`.
        if cfg.same_site == SameSite::None && !cfg.secure {
            panic!("SESSION_SAME_SITE=none requires SESSION_SECURE_COOKIE=true");
        }
        cfg
    }
}
