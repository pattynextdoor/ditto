use std::path::Path;

use anyhow::Result;
use console::style;
use similar::{ChangeTag, TextDiff};

use crate::config::{DittoConfig, Package};
use crate::paths::expand_tilde;
use crate::ui::Ui;

pub fn run(config: &DittoConfig, root: &Path, packages: &[String], ui: &Ui) -> Result<()> {
    let packages_to_diff: Vec<(&String, &Package)> = if packages.is_empty() {
        config.packages.iter().collect()
    } else {
        config
            .packages
            .iter()
            .filter(|(name, _)| packages.contains(name))
            .collect()
    };

    let mut has_diffs = false;

    for (name, package) in packages_to_diff {
        for mapping in &package.files {
            let source = root.join(&mapping.src);
            let target = expand_tilde(&mapping.target)?;

            // Skip if either file doesn't exist
            if !source.exists() || !target.exists() {
                continue;
            }

            // Read both files; skip binary/unreadable files
            let Ok(source_content) = std::fs::read_to_string(&source) else {
                continue;
            };
            let Ok(target_content) = std::fs::read_to_string(&target) else {
                continue;
            };

            if source_content == target_content {
                continue;
            }

            has_diffs = true;
            println!(
                "\n{}  {} --> {}",
                style("[diff]").magenta().bold(),
                name,
                target.display()
            );

            let diff = TextDiff::from_lines(&source_content, &target_content);
            for change in diff.iter_all_changes() {
                match change.tag() {
                    ChangeTag::Delete => print!("{}", style(format!("- {change}")).red()),
                    ChangeTag::Insert => print!("{}", style(format!("+ {change}")).green()),
                    ChangeTag::Equal => print!("  {change}"),
                }
            }
        }
    }

    if !has_diffs {
        ui.info("Everything in sync.");
    }

    Ok(())
}
