use std::fs;
use std::path::{Path, PathBuf};
#[derive(Debug)]
pub struct File;

impl File {
    /// Create a symbolic link between source and destination paths
    ///
    /// # Arguments
    ///
    /// * `source` - The source path to link from
    /// * `destination` - The destination path to link to
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use rustavel_core::helper::file::File;
    /// File::create_link("storage/app/public", "public/storage").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `io::Error` if the link creation fails
    /// # Note:
    /// ⚠️ This operation has filesystem side effects and is platform-dependent.
    /// so ignored in test
    ///
    pub fn create_link(source: &str, destination: &str) -> std::io::Result<()> {
        let source_path = Path::new(source);
        let destination_path = Path::new(destination);

        // check if file exists before
        if destination_path.exists() {
            fs::remove_file(destination_path)?;
        }

        // create symbolic link
        #[cfg(unix)]
        std::os::unix::fs::symlink(source_path, destination_path)?;

        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(source_path, destination_path)?;

        Ok(())
    }
}

/// Returns the project root directory.
///
/// Attempts to retrieve the project root from `CARGO_MANIFEST_DIR`.
/// If the environment variable is not set or contains an invalid path,
/// falls back to the current working directory.
pub fn project_root() -> Result<PathBuf, std::io::Error> {
    if let Some(path) = std::env::var_os("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(path);
        path.pop();

        if path.is_dir() {
            return Ok(path);
        }
    }

    std::env::current_dir()
}
