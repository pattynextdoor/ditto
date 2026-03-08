use std::path::Path;

use anyhow::Result;

use crate::ui::Ui;

pub fn run(url: &str, path: &Path, packages: &[String], dry_run: bool, ui: &Ui) -> Result<()> {
    ui.banner();

    let status = std::process::Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(path)
        .status()?;

    if !status.success() {
        anyhow::bail!("git clone failed");
    }

    let config = crate::config::load(&path.join("ditto.toml"))?;
    crate::ops::link::run(&config, path, packages, false, dry_run, &ui)?;
    ui.success("Ditto transformed into your dev environment!");
    Ok(())
}
