use std::path::Path;

use anyhow::Result;

use crate::backup;
use crate::config::{DittoConfig, Package};
use crate::hooks;
use crate::paths::expand_tilde;
use crate::symlink::{is_symlink_to, remove_symlink};
use crate::ui::Ui;

pub fn run(
    config: &DittoConfig,
    root: &Path,
    packages: &[String],
    all: bool,
    dry_run: bool,
    ui: &Ui,
) -> Result<()> {
    if packages.is_empty() && !all {
        ui.warning("No packages specified to unlink. Use --all to unlink everything.");
        return Ok(());
    }

    let packages_to_unlink: Vec<(&String, &Package)> = if all {
        config.packages.iter().collect()
    } else {
        config
            .packages
            .iter()
            .filter(|(name, _)| packages.contains(name))
            .collect()
    };

    let backup_dir = root.join(&config.settings.backup_dir);

    for (name, package) in packages_to_unlink {
        // Run pre-unlink hooks
        if let Some(hooks) = &package.hooks {
            if let Some(cmd) = &hooks.pre_unlink {
                hooks::run_hooks(cmd)?;
            }
        }

        // Remove symlinks from mapped package files
        for mapping in &package.files {
            let source = root.join(&mapping.src);
            let target = expand_tilde(&mapping.target)?;

            if is_symlink_to(&target, &source) {
                if dry_run {
                    ui.dry_run(&format!("would remove symlink {}", target.display()));
                } else {
                    remove_symlink(&target)?;
                    ui.success(&format!("[{}] {} unlinked", name, target.display()));

                    // Only try to restore if there's a backup
                    if backup::has_backup(&target, &backup_dir)? {
                        backup::restore(&target, &backup_dir)?;
                        ui.success(&format!("[{}] {} restored from backup", name, target.display()));
                    }
                }
            }
        }

        // Run post-unlink hooks
        if let Some(hooks) = &package.hooks {
            if let Some(cmd) = &hooks.post_unlink {
                hooks::run_hooks(cmd)?;
            }
        }
    }
    ui.success("Symlinks removed.");
    Ok(())
}
