use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ditto", about = "A dotfile manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable dry-run mode
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Show detailed output
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Path to a specific ditto.toml
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show which mappings are linked, broken or untracked
    Status,

    /// Clone a dotfiles repo and link everything (or a subset)
    Init {
        /// Git URL of the dotfiles repo
        url: String,

        /// Clone destination
        #[arg(long, default_value = "~/.dotfiles")]
        path: PathBuf,

        /// Only link specific packages
        #[arg(long)]
        packages: Vec<String>,
    },

    /// Create symlinks
    Link {
        /// List of packages to link
        packages: Vec<String>,

        /// Overwrite conflicts
        #[arg(long)]
        force: bool,
    },

    /// Remove symlinks and restore backed-up originals
    Unlink {
        /// List of packages to unlink
        packages: Vec<String>,

        /// Unlink all packages
        #[arg(long)]
        all: bool,
    },

    /// Move an existing file into the repo and replace it with a symlink
    Add {
        /// Path to the file to add
        path: PathBuf,

        /// Target package name
        #[arg(long)]
        package: String,
    },

    /// Show text diffs between repo files and live files
    Diff {
        /// Packages to diff
        packages: Vec<String>,
    },
}
