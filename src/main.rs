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

    match cli.command {
        cli::Commands::Status => {
            println!("status");
        }
        cli::Commands::Init { url, path, packages } => {
            println!("init: url={url}, path={path:?}, packages={packages:?}");
        }
        cli::Commands::Link { packages, force } => {
            println!("link: packages={packages:?}, force={force}");
        }
        cli::Commands::Unlink { packages, all } => {
            println!("unlink: packages={packages:?}, all={all}");
        }
        cli::Commands::Add { path, package } => {
            println!("add: path={path:?}, package={package}");
        }
        cli::Commands::Diff { packages } => {
            println!("diff: packages={packages:?}");
        }
    }

    Ok(())
}
