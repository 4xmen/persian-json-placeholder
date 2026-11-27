use rustavel_core::config::CONFIG;
use serde::Serialize;

pub fn asset_url(path: Option<&str>) -> Option<String> {
    path.map(|path| {
        format!(
            "{}/{}",
            CONFIG.app.url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    })
}
