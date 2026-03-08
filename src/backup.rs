use std::path::{Path, PathBuf};

use anyhow::Result;

pub fn backup_path_for(target: &Path, backup_dir: &Path) -> Result<PathBuf> {
    let relative = crate::paths::relative_to_home(target)?;
    Ok(backup_dir.join(relative))
}

pub fn has_backup(target: &Path, backup_dir: &Path) -> Result<bool> {
    Ok(backup_path_for(target, backup_dir)?.exists())
}

pub fn backup(target: &Path, backup_dir: &Path) -> Result<()> {
    let backup_path = backup_path_for(target, backup_dir)?;

    if let Some(parent) = backup_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::copy(target, backup_path)?;
    Ok(())
}

pub fn restore(target: &Path, backup_dir: &Path) -> Result<()> {
    let backup_path = backup_path_for(target, backup_dir)?;

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::rename(backup_path, target)?;
    Ok(())
}
