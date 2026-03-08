// TODO: You'll implement config parsing here
// This module handles ditto.toml parsing and schema types
//
// Needed types:
// - DittoConfig (top-level)
// - Settings (backup_dir, etc.)
// - Package (files, hooks, platforms)
// - FileMapping (src, target)
// - Hooks (pre_link, post_link, pre_unlink, post_unlink)
//
// Needed functions:
// - load(path) -> Result<DittoConfig>
// - find_root() -> Result<PathBuf>  (walk up from CWD looking for ditto.toml)
