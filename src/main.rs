mod backup;
mod cli;
mod config;
mod error;
mod hooks;
mod ops;
mod paths;
mod symlink;
mod ui;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let ui = ui::Ui::new(cli.no_color, cli.dry_run);

    match cli.command {
        cli::Commands::Init {
            url,
            path,
            packages,
        } => {
            let path = paths::expand_tilde(&path.to_string_lossy())?;
            ops::init::run(&url, &path, &packages, cli.dry_run, &ui)?;
        }
        cli::Commands::Status => {
            let root = config::find_root()?;
            let config = config::load(&root.join("ditto.toml"))?;
            ops::status::run(&config, &root, &ui)?;
        }
        cli::Commands::Link { packages, force } => {
            let root = config::find_root()?;
            let config = config::load(&root.join("ditto.toml"))?;
            ops::link::run(&config, &root, &packages, force, cli.dry_run, &ui)?;
        }
        cli::Commands::Unlink { packages, all } => {
            let root = config::find_root()?;
            let config = config::load(&root.join("ditto.toml"))?;
            ops::unlink::run(&config, &root, &packages, all, cli.dry_run, &ui)?;
        }
        cli::Commands::Add { path, package } => {
            let root = config::find_root()?;
            let config_path = root.join("ditto.toml");
            ops::add::run(&config_path, &root, &path, &package, cli.dry_run, &ui)?;
        }
        cli::Commands::Diff { packages } => {
            let root = config::find_root()?;
            let config = config::load(&root.join("ditto.toml"))?;
            ops::diff::run(&config, &root, &packages, &ui)?;
        }
    }

    Ok(())
}
