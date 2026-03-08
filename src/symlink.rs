use std::path::Path;

use anyhow::Result;

pub fn create_symlink(src: &Path, target: &Path) -> Result<()> {
    std::os::unix::fs::symlink(src, target)?;
    Ok(())
}

pub fn is_symlink_to(path: &Path, expected: &Path) -> bool {
    if let Ok(link_target) = std::fs::read_link(path) {
        link_target == expected
    } else {
        false
    }
}

pub fn remove_symlink(path: &Path) -> Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}
