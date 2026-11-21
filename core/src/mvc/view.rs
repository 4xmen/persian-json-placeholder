use crate::config::CONFIG;
use minijinja::value::Serde;
use minijinja::{Environment, Error, ErrorKind, State, Value};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ViewsDirError {
    #[error("Could not locate views directory. Checked paths: {0:?}")]
    NotFound(Vec<PathBuf>),
}

pub fn resolve_views_dir() -> Result<PathBuf, ViewsDirError> {
    let mut candidates = Vec::new();

    // 1. Environment variable
    if let Some(path) = std::env::var_os("APP_VIEWS_PATH") {
        candidates.push(PathBuf::from(path));
    }

    // 2. Directory next to executable
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        candidates.push(exe_dir.join("resources").join("views"));
    }

    // 3. Development fallback
    candidates.push(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate directory has no parent")
            .join("app")
            .join("src")
            .join("resources")
            .join("views"),
    );

    // Find first existing directory
    for path in &candidates {
        if path.is_dir() {
            return Ok(path.clone());
        }
    }

    Err(ViewsDirError::NotFound(candidates))
}

pub fn register_global_func(env: &mut Environment<'static>) {
    env.add_function("url", || CONFIG.app.url.clone());

    env.add_function("config", |path: String| -> Result<Value, Error> {
        let value = CONFIG.get(&path).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                format!("config key not found: {path}"),
            )
        })?;

        Ok(Value::from(Serde(value)))
    });
}

/// Registers `csrf_field()` and `csrf_meta()` as template globals.
///
/// Both read `csrf_token` from the render context (see `AppState::render`),
/// because the `Environment` is shared across requests while the token is per-session.
/// They fail loudly when the token is missing instead of emitting an empty value.
pub fn register_csrf_functions(env: &mut Environment<'static>) {
    // {{ csrf_field() }} -> <input type="hidden" name="_token" value="...">
    env.add_function("csrf_field", |state: &State| -> Result<Value, Error> {
        let token = csrf_from_state(state)?;
        // The token is base64url (A-Z a-z 0-9 - _), so it is safe inside an attribute.
        Ok(Value::from_safe_string(format!(
            r#"<input type="hidden" name="_token" value="{token}">"#
        )))
    });

    // {{ csrf_meta() }} -> <meta name="csrf-token" content="...">  (for fetch/XHR)
    env.add_function("csrf_meta", |state: &State| -> Result<Value, Error> {
        let token = csrf_from_state(state)?;
        Ok(Value::from_safe_string(format!(
            r#"<meta name="csrf-token" content="{token}">"#
        )))
    });
}

fn csrf_from_state(state: &State) -> Result<String, Error> {
    match state.lookup("csrf_token") {
        Some(v) if !v.is_undefined() && !v.is_none() => Ok(v.to_string()),
        _ => Err(Error::new(
            ErrorKind::InvalidOperation,
            "csrf_token is missing from the render context; render through AppState::render()",
        )),
    }
}
