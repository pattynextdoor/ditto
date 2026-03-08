use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DittoError {
    #[error("Config file not found. Run `ditto init` first or create a ditto.toml")]
    ConfigNotFound,

    #[error("Failed to parse config: {0}")]
    ConfigParse(String),

    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("File already exists and is not a symlink: {}", .0.display())]
    ConflictExists(PathBuf),

    #[error("Symlink target does not exist: {}", .0.display())]
    SourceNotFound(PathBuf),

    #[error("Backup failed for {}: {message}", .path.display())]
    BackupFailed { path: PathBuf, message: String },

    #[error("Hook failed for package '{package}': {message}")]
    HookFailed { package: String, message: String },

    #[error("Git command failed: {0}")]
    GitError(String),

    #[error("Path is not inside home directory: {}", .0.display())]
    NotInHome(PathBuf),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
