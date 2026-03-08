use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DittoConfig {
    pub settings: Settings,
    pub packages: HashMap<String, Package>,
}

#[derive(Deserialize)]
pub struct Settings {
    pub backup_dir: String,
}

#[derive(Deserialize)]
pub struct Package {
    pub files: Vec<FileMapping>,
    pub hooks: Option<Hooks>,
    pub platforms: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct FileMapping {
    pub src: String,
    pub target: String,
}

#[derive(Deserialize)]
pub struct Hooks {
    pub pre_link: Option<String>,
    pub post_link: Option<String>,
    pub pre_unlink: Option<String>,
    pub post_unlink: Option<String>,
}

pub fn load(path: &Path) -> Result<DittoConfig> {
    let file_contents = fs::read_to_string(path)?;

    let config: DittoConfig = toml::from_str(&file_contents)?;
    Ok(config)
}

pub fn find_root() -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        if current_dir.join("ditto.toml").exists() {
            return Ok(current_dir);
        }

        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    // fallback: ~/.dotfiles
    let fallback = crate::paths::home_dir()?.join(".dotfiles");
    if fallback.join("ditto.toml").exists() {
        return Ok(fallback);
    }

    anyhow::bail!("Could not find ditto.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_parses_valid_ditto_toml() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("ditto.toml");
        std::fs::write(
            &toml_path,
            r#"
[settings]
backup_dir = ".ditto-backup"

[packages.shell]
files = [
  { src = "shell/zshrc", target = "~/.zshrc" },
]

[packages.shell.hooks]
post_link = "source ~/.zshrc"

[packages.git]
files = [
  { src = "git/gitconfig", target = "~/.gitconfig" },
]
"#,
        )
        .unwrap();

        let config = load(&toml_path).unwrap();
        assert_eq!(config.settings.backup_dir, ".ditto-backup");
        assert_eq!(config.packages.len(), 2);
        assert!(config.packages.contains_key("shell"));
        assert!(config.packages.contains_key("git"));
    }

    #[test]
    fn load_parses_package_with_hooks() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("ditto.toml");
        std::fs::write(
            &toml_path,
            r#"
[settings]
backup_dir = ".ditto-backup"

[packages.ssh]
files = [
  { src = "ssh/", target = "~/.ssh/" },
]

[packages.ssh.hooks]
post_link = "chmod 700 ~/.ssh"
pre_unlink = "echo unlinking ssh"
"#,
        )
        .unwrap();

        let config = load(&toml_path).unwrap();
        let ssh = &config.packages["ssh"];
        let hooks = ssh.hooks.as_ref().unwrap();
        assert_eq!(hooks.post_link.as_deref(), Some("chmod 700 ~/.ssh"));
        assert_eq!(hooks.pre_unlink.as_deref(), Some("echo unlinking ssh"));
        assert!(hooks.pre_link.is_none());
        assert!(hooks.post_unlink.is_none());
    }

    #[test]
    fn load_parses_package_with_platform_filter() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("ditto.toml");
        std::fs::write(
            &toml_path,
            r#"
[settings]
backup_dir = ".ditto-backup"

[packages.iterm2]
platforms = ["macos"]
files = [
  { src = "iterm2/prefs.plist", target = "~/Library/Preferences/prefs.plist" },
]
"#,
        )
        .unwrap();

        let config = load(&toml_path).unwrap();
        let iterm = &config.packages["iterm2"];
        assert_eq!(iterm.platforms.as_deref(), Some(&["macos".to_string()][..]));
    }

    #[test]
    fn load_fails_on_invalid_toml() {
        let dir = tempfile::tempdir().unwrap();
        let toml_path = dir.path().join("ditto.toml");
        std::fs::write(&toml_path, "this is not valid toml [[[").unwrap();

        let result = load(&toml_path);
        assert!(result.is_err());
    }

    #[test]
    fn load_fails_on_missing_file() {
        let result = load(Path::new("/nonexistent/ditto.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn find_root_finds_ditto_toml_in_current_dir() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("ditto.toml"),
            "[settings]\nbackup_dir = \".backup\"\n[packages]\n",
        )
        .unwrap();

        // Change to the temp dir to test find_root
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        let result = find_root();
        std::env::set_current_dir(original_dir).unwrap();

        assert_eq!(
            result.unwrap().canonicalize().unwrap(),
            dir.path().canonicalize().unwrap()
        );
    }
}
