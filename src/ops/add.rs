use std::path::Path;

use anyhow::Result;
use toml_edit::DocumentMut;

use crate::paths::{expand_tilde, relative_to_home};
use crate::symlink::create_symlink;
use crate::ui::Ui;

pub fn run(
    config_path: &Path,
    root: &Path,
    file_path: &Path,
    package: &str,
    dry_run: bool,
    ui: &Ui,
) -> Result<()> {
    // Resolve the file path (handle ~)
    let file_path = if file_path.starts_with("~") {
        expand_tilde(&file_path.to_string_lossy())?
    } else {
        file_path.to_path_buf()
    };

    // Verify the file exists
    if !file_path.exists() {
        anyhow::bail!("File not found: {}", file_path.display());
    }

    // Compute where it goes in the repo: root/package/filename
    let file_name = file_path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Could not determine filename for {}", file_path.display()))?;
    let dest = root.join(package).join(file_name);

    // The src path relative to the repo root (e.g. "shell/zshrc")
    let src_relative = format!("{}/{}", package, file_name.to_string_lossy());

    // The target path with ~ prefix for the toml entry (e.g. "~/.zshrc")
    let target_relative = format!("~/{}", relative_to_home(&file_path)?.display());

    if dry_run {
        ui.dry_run(&format!("would move {} into repo at {}", file_path.display(), dest.display()));
        ui.dry_run(&format!("would link {} --> {}", target_relative, src_relative));
        ui.dry_run(&format!("would update config at {}", config_path.display()));
        return Ok(());
    }

    // Create the package directory in the repo if needed
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Move the file into the repo
    std::fs::rename(&file_path, &dest)?;
    ui.success(&format!("moved {} into repo at {}", file_path.display(), dest.display()));

    // Replace with a symlink pointing back to the repo copy
    create_symlink(&dest, &file_path)?;
    ui.success(&format!("linked {} --> {}", file_path.display(), dest.display()));

    // Update ditto.toml with the new file mapping
    let content = std::fs::read_to_string(config_path)?;
    let mut doc = content.parse::<DocumentMut>()?;

    let files = doc["packages"][package]["files"]
        .as_array_mut()
        .ok_or_else(|| anyhow::anyhow!("Could not find files array for package '{}'", package))?;

    let mut mapping = toml_edit::InlineTable::new();
    mapping.insert("src", src_relative.into());
    mapping.insert("target", target_relative.into());
    files.push(toml_edit::Value::InlineTable(mapping));

    std::fs::write(config_path, doc.to_string())?;
    ui.success(&format!("updated config at {}", config_path.display()));

    Ok(())
}
