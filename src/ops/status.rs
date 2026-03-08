use std::path::Path;

use anyhow::Result;

use crate::config::DittoConfig;
use crate::paths::expand_tilde;
use crate::symlink::is_symlink_to;
use crate::ui::Ui;

pub fn run(config: &DittoConfig, root: &Path, ui: &Ui) -> Result<()> {
    ui.info("Checking status...");
    for (name, package) in &config.packages {
        for mapping in &package.files {
            let source = root.join(&mapping.src);
            let target = expand_tilde(&mapping.target)?;

            if is_symlink_to(&target, &source) {
                ui.success(&format!("{} --> {} [linked]", name, target.display()));
            } else if target.exists() {
                ui.warning(&format!("{} --> {} [conflict: file exists]", name, target.display()));
            } else {
                ui.info(&format!("{} --> {} [not linked]", name, target.display()));
            }
        }
    }
    Ok(())
}
