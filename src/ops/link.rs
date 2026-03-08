use std::path::Path;

use anyhow::Result;

use crate::backup;
use crate::config::{DittoConfig, Package};
use crate::hooks;
use crate::paths::{current_platform, expand_tilde};
use crate::symlink::{create_symlink, is_symlink_to};
use crate::ui::Ui;

pub fn run(
    config: &DittoConfig,
    root: &Path,
    packages: &[String],
    force: bool,
    dry_run: bool,
    ui: &Ui,
) -> Result<()> {
    // If no packages specified, link all
    let packages_to_link: Vec<(&String, &Package)> = if packages.is_empty() {
        config.packages.iter().collect()
    } else {
        config
            .packages
            .iter()
            .filter(|(name, _)| packages.contains(name))
            .collect()
    };

    for (name, package) in packages_to_link {
        // Verify platforms
        if let Some(platforms) = &package.platforms {
            let current = current_platform().to_string();

            if !platforms.contains(&current) {
                ui.info(&format!("skipping {} (wrong platform)", name));
                continue;
            }
        }

        // Run pre-link hooks
        if let Some(hooks) = &package.hooks {
            if let Some(cmd) = &hooks.pre_link {
                hooks::run_hooks(cmd)?;
            }
        }

        // Map package files
        for mapping in &package.files {
            let source = root.join(&mapping.src);
            let target = expand_tilde(&mapping.target)?;

            // Skip already-linked files
            if is_symlink_to(&target, &source) {
                ui.info(&format!("{} already linked", target.display()));
                continue;
            }

            // Detect conflict
            if target.exists() {
                if force {
                    if dry_run {
                        ui.dry_run(&format!("would backup {}", target.display()));
                    } else {
                        // Backup then remove
                        let backup_dir = root.join(&config.settings.backup_dir);
                        backup::backup(&target, &backup_dir)?;
                        std::fs::remove_file(&target)?;
                    }
                } else {
                    ui.warning(&format!(
                        "{} conflict, use --force to overwrite",
                        target.display()
                    ));
                    continue;
                }
            }

            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if dry_run {
                ui.dry_run(&format!("would link {} --> {}", name, target.display()));
            } else {
                create_symlink(&source, &target)?;
                ui.success(&format!(
                    "{} --> {} symlink created",
                    name,
                    target.display()
                ))
            }
        }

        // Run post-link hooks
        if let Some(hooks) = &package.hooks {
            if let Some(cmd) = &hooks.post_link {
                hooks::run_hooks(cmd)?;
            }
        }
    }

    Ok(())
}
