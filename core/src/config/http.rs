use crate::config::env::{env_bool, env_parse};
use serde::Serialize;
use std::env;

/// HTTP layer configuration (middleware stack defaults).
///
/// Every value has a safe default and can be overridden from `.env`,
/// so the end user never has to touch `main.rs` to tune the stack.
#[derive(Debug, Clone, Serialize)]
pub struct HttpConfig {
    /// Max request body size in bytes (HTTP_MAX_BODY_SIZE).
    /// Protects against memory-exhaustion DoS via huge bodies/uploads.
    /// NOTE: this replaces the old `app.max_upload_size`.
    pub max_body_size: usize,

    /// Max time (seconds) allowed to process one request (HTTP_TIMEOUT_SECS).
    /// Protects against slow/hanging requests. Returns 408 on expiry.
    pub timeout_secs: u64,

    /// Enable gzip/br/zstd response compression (HTTP_COMPRESSION).
    pub compression: bool,

    /// Enable the CSRF guard (HTTP_CSRF_GUARD).
    /// Checks `Sec-Fetch-Site` / `Origin` on unsafe methods (POST, PUT, ...).
    /// This is needed for cookie-based sessions (web forms).
    /// It is NOT needed for JWT sent via the `Authorization` header,
    /// because browsers do not attach that header automatically.
    pub csrf_guard: bool,

    /// Send `Strict-Transport-Security` (HTTP_HSTS).
    /// Only enable behind HTTPS, otherwise it is meaningless/harmful.
    pub hsts: bool,

    pub cors: CorsConfig,
}

/// CORS configuration.
///
/// IMPORTANT: CORS is NOT a CSRF protection. CORS only controls whether a page
/// from another origin may READ the response. A cross-site `<form method="POST">`
/// is still sent by the browser (with cookies) without any CORS check.
/// That is why CSRF is handled separately by `csrf_guard`.
#[derive(Debug, Clone, Serialize)]
pub struct CorsConfig {
    /// Comma separated list (CORS_ALLOWED_ORIGINS), e.g.
    /// `https://app.example.com,https://admin.example.com`
    /// Empty = CORS layer disabled (same-origin only). We never use `permissive`.
    pub allowed_origins: Vec<String>,

    /// Allow cookies/credentials on cross-origin requests (CORS_ALLOW_CREDENTIALS).
    pub allow_credentials: bool,

    /// How long browsers may cache the preflight result (CORS_MAX_AGE_SECS).
    pub max_age_secs: u64,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            max_body_size: 2 * 1024 * 1024, // 2 MiB
            timeout_secs: 30,
            compression: true,
            csrf_guard: true,
            hsts: false,
            cors: CorsConfig::default(),
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec![],
            allow_credentials: false,
            max_age_secs: 3600,
        }
    }
}

impl HttpConfig {
    pub fn from_env() -> Self {
        let d = Self::default();

        Self {
            max_body_size: env_parse("HTTP_MAX_BODY_SIZE", d.max_body_size),
            timeout_secs: env_parse("HTTP_TIMEOUT_SECS", d.timeout_secs),
            compression: env_bool("HTTP_COMPRESSION", d.compression),
            csrf_guard: env_bool("HTTP_CSRF_GUARD", d.csrf_guard),
            hsts: env_bool("HTTP_HSTS", d.hsts),
            cors: CorsConfig::from_env(),
        }
    }
}

impl CorsConfig {
    pub fn from_env() -> Self {
        let d = Self::default();

        let allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .map(|v| {
                v.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or(d.allowed_origins);

        Self {
            allowed_origins,
            allow_credentials: env_bool("CORS_ALLOW_CREDENTIALS", d.allow_credentials),
            max_age_secs: env_parse("CORS_MAX_AGE_SECS", d.max_age_secs),
        }
    }
}
