use anyhow::Result;

use std::path::Path;
use std::path::PathBuf;

use crate::error::DittoError;

pub enum Platform {
    MacOs,
    Linux,
}

pub fn current_platform() -> Platform {
    match std::env::consts::OS {
        "macos" => Platform::MacOs,
        "linux" => Platform::Linux,
        _ => panic!("Ditto is not supported on your platform.")
    }
}

pub fn home_dir() -> Result<PathBuf> {
    let dir = dirs::home_dir();

    match dir {
        Some(dir) => Ok(dir),
        None => Err(anyhow::anyhow!("Home directory not set."))
    }
}

pub fn relative_to_home(path: &Path) -> Result<PathBuf> {
    let home = home_dir()?;

    match path.strip_prefix(&home) {
        Ok(relative) => Ok(relative.to_path_buf()),
        Err(_) => Err(DittoError::NotInHome(path.to_path_buf()).into())
    }
}

pub fn expand_tilde(path: &str) -> Result<PathBuf> {
    if path.starts_with("~") {
        let rest = &path[2..];

        Ok(home_dir()?.join(rest))
    } else {
        Ok(PathBuf::from(path))
    }
}
