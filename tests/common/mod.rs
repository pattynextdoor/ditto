use std::path::{Path, PathBuf};

/// Create a temp directory under $HOME so backup path resolution works.
/// The returned TempDir cleans itself up on drop -- no leftover directories.
pub fn home_tempdir() -> tempfile::TempDir {
    let home = dirs::home_dir().unwrap();
    tempfile::Builder::new()
        .prefix(".ditto-test-")
        .tempdir_in(home)
        .unwrap()
}

/// Create a minimal dotfiles repo structure in a temp directory.
pub fn setup_dotfiles_repo(root: &Path, targets: &[(String, String)]) -> PathBuf {
    let mut toml = String::from("[settings]\nbackup_dir = \".ditto-backup\"\n\n");

    for (pkg_name, target_path) in targets {
        let src_filename = Path::new(target_path)
            .file_name()
            .unwrap()
            .to_string_lossy();
        let src_path = format!("{pkg_name}/{src_filename}");

        // Create the source file in the repo
        let src_full = root.join(&src_path);
        std::fs::create_dir_all(src_full.parent().unwrap()).unwrap();
        std::fs::write(&src_full, format!("# managed by ditto ({pkg_name})")).unwrap();

        toml.push_str(&format!(
            "[packages.{pkg_name}]\nfiles = [\n  {{ src = \"{src_path}\", target = \"{target_path}\" }},\n]\n\n"
        ));
    }

    std::fs::write(root.join("ditto.toml"), toml).unwrap();
    root.to_path_buf()
}
