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
    let file_contents = fs::read_to_string(&path)?;

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
