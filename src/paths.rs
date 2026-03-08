use anyhow::Result;

use std::path::Path;
use std::path::PathBuf;

use crate::error::DittoError;

pub enum Platform {
    MacOs,
    Linux,
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::MacOs => write!(f, "macos"),
            Platform::Linux => write!(f, "linux"),
        }
    }
}

pub fn current_platform() -> Platform {
    match std::env::consts::OS {
        "macos" => Platform::MacOs,
        "linux" => Platform::Linux,
        _ => panic!("Ditto is not supported on your platform."),
    }
}

pub fn home_dir() -> Result<PathBuf> {
    let dir = dirs::home_dir();

    match dir {
        Some(dir) => Ok(dir),
        None => Err(anyhow::anyhow!("Home directory not set.")),
    }
}

pub fn relative_to_home(path: &Path) -> Result<PathBuf> {
    let home = home_dir()?;

    match path.strip_prefix(&home) {
        Ok(relative) => Ok(relative.to_path_buf()),
        Err(_) => Err(DittoError::NotInHome(path.to_path_buf()).into()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_tilde_converts_home_prefix_to_absolute_path() {
        let result = expand_tilde("~/.zshrc").unwrap();
        let home = home_dir().unwrap();
        assert_eq!(result, home.join(".zshrc"));
    }

    #[test]
    fn expand_tilde_leaves_absolute_paths_unchanged() {
        let result = expand_tilde("/etc/hosts").unwrap();
        assert_eq!(result, PathBuf::from("/etc/hosts"));
    }

    #[test]
    fn relative_to_home_strips_home_prefix() {
        let home = home_dir().unwrap();
        let path = home.join(".config/ditto");
        let result = relative_to_home(&path).unwrap();
        assert_eq!(result, PathBuf::from(".config/ditto"));
    }

    #[test]
    fn relative_to_home_rejects_paths_outside_home() {
        let result = relative_to_home(Path::new("/tmp/outside"));
        assert!(result.is_err());
    }

    #[test]
    fn platform_displays_as_lowercase_string() {
        let platform = current_platform();
        let display = platform.to_string();
        assert!(display == "macos" || display == "linux");
    }
}
