<p align="center">

<h3>Ditto transforms into your dev environment.</h3>

[![Rust](https://img.shields.io/badge/Built_with-Rust-dea584?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)
[![Status](https://img.shields.io/badge/Status-WIP-orange?style=flat-square)](#status)

</p>

---

A Dotfile manager CLI built as a Rust learning project with [Claude Code](https://claude.ai/code) as a coding partner (using the [`learning-opportunities`](https://github.com/DrCatHicks/learning-opportunities) skill). All core logic was hand-typed; Claude served as a teacher, not an author. You could use it too if you want.

Extends upon GNU Stow functionality with support for hook setup. And has pretty spinners.

## Install

```sh
cargo install --path .
```

## Usage

**Set up a new machine:**

```sh
ditto init git@github.com:you/dotfiles.git   # clone + link everything
ditto init <URL> --packages shell git         # only link what you need
```

**Link and unlink:**

```sh
ditto link                    # symlink all packages
ditto link shell ssh          # just these two
ditto link --force            # overwrite conflicts (backs up first)
ditto unlink ssh              # remove symlinks, restore originals
ditto unlink --all            # tear it all down
```

**Bring an existing file under management:**

```sh
ditto add ~/.zshrc --package shell   # move into repo, replace with symlink
```

**See what's going on:**

```sh
ditto status                  # linked, broken, untracked -- at a glance
ditto diff                    # what changed since you linked?
```

## Config

A `ditto.toml` at the root of your dotfiles repo. That's the whole system.

```toml
[settings]
backup_dir = ".ditto-backup"

[packages.shell]
files = [
  { src = "shell/zshrc", target = "~/.zshrc" },
]

[packages.shell.hooks]
post_link = "source ~/.zshrc"

[packages.ssh]
files = [
  { src = "ssh/", target = "~/.ssh/" },
]

[packages.ssh.hooks]
post_link = "chmod 700 ~/.ssh && chmod 600 ~/.ssh/id_*"

[packages.iterm2]
platforms = ["macos"]
files = [
  { src = "iterm2/com.googlecode.iterm2.plist", target = "~/Library/Preferences/com.googlecode.iterm2.plist" },
]
```

Packages group related files. Each can have platform filters and pre/post hooks for both linking and unlinking.

## Why ditto?

| Feature | How it works |
|---------|-------------|
| **Package-based** | Group files logically -- `shell`, `git`, `ssh` -- link what you need |
| **Hooks** | Run commands after linking (reload shell, fix permissions) |
| **Automatic backup** | Conflicts are backed up before overwriting, restored on unlink |
| **Dry run** | `--dry-run` on any command to preview before committing |
| **No templating** | Same config everywhere -- just symlinks, done right |

## Global Flags

```
--dry-run       Preview without making changes
--verbose       Show detailed output
--no-color      Disable colored output
--config <PATH> Use a specific ditto.toml
```

## Status

Ditto is a **work in progress**. The project structure and core utilities are in place; command implementations are actively being built. macOS and Linux only.

## License

[MIT](LICENSE)
