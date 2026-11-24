use crate::make::make_error::MakeError;
use data_encoding::BASE64;
use rand::TryRng;
use rand::rngs::SysRng;
use rustavel_core::helper::file::project_root;
use std::fs;
use std::path::{Path, PathBuf};

/// Generate a secure Laravel-style `APP_KEY` for the `.env` file.
///
/// This function does:
/// 1. Generate 32 random bytes using a cryptographically secure RNG (`SysRng`).
/// 2. Encode the bytes in Base64.
/// 3. Prefix the result with `base64:` to match Laravel's expected format.
/// 4. Return the complete string, ready to be written to `.env`.
pub fn generate_laravel_app_key() -> String {
    let mut key = [0u8; 32];
    SysRng.try_fill_bytes(&mut key).unwrap();

    format!("base64:{}", BASE64.encode(&key))
}

/// Insert or update a key-value pair in the `.env` file in the current directory.
///
/// This function does:
/// 1. Resolve the `.env` path relative to the current working directory.
/// 2. If the `.env` file exists, read its content; otherwise start with empty content.
/// 3. Search for the given key:
///    - If found, replace its value with the new one.
///    - If not found, append `key=value` to the end of the file.
/// 4. Write the updated content back to `.env`, creating the file if necessary.
pub fn set_env_value(key: &str, value: &str) -> std::io::Result<()> {
    let env_path: PathBuf = project_root()?.join(".env");

    let mut content = if env_path.exists() {
        fs::read_to_string(&env_path)?
    } else {
        String::new()
    };

    let mut found = false;

    content = content
        .lines()
        .map(|line| {
            if line.starts_with(&format!("{}=", key)) {
                found = true;
                format!("{}={}", key, value)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if !found {
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(&format!("{}={}", key, value));
    }

    fs::write(env_path, content)
}

/// Ensure a module is registered in mod.rs (idempotent)
pub async fn register_mod_file(mod_file: &Path, module_name: &str) -> Result<(), MakeError> {
    // Read existing mod.rs content or create empty if missing
    let content = tokio::fs::read_to_string(mod_file)
        .await
        .unwrap_or_else(|_| String::new());

    let line = format!("pub mod {};", module_name);

    // Avoid duplicate registration
    if content.lines().any(|l| l.trim() == line) {
        return Ok(());
    }

    // Append module declaration
    let mut new_content = content;
    if !new_content.ends_with('\n') && !new_content.is_empty() {
        new_content.push('\n');
    }
    new_content.push_str(&line);
    new_content.push('\n');

    tokio::fs::write(mod_file, new_content).await?;
    Ok(())
}

/// Refuses to overwrite a file the developer may already have edited.
pub fn ensure_new_file(path: &Path) -> Result<(), MakeError> {
    if path.exists() {
        return Err(MakeError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("{} already exists", path.display()),
        )));
    }
    Ok(())
}
