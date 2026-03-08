mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::home_tempdir;

#[test]
fn add_moves_file_into_repo_and_creates_symlink() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = home_tempdir();

    std::fs::write(
        repo_dir.path().join("ditto.toml"),
        "[settings]\nbackup_dir = \".ditto-backup\"\n\n[packages.shell]\nfiles = []\n",
    )
    .unwrap();

    let file_to_add = target_dir.path().join("zshrc");
    std::fs::write(&file_to_add, "# my zshrc").unwrap();

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("add")
        .arg(&file_to_add)
        .arg("--package")
        .arg("shell")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("moved"))
        .stdout(predicate::str::contains("symlinked"));

    // File is in the repo
    assert!(repo_dir.path().join("shell/zshrc").exists());
    assert_eq!(
        std::fs::read_to_string(repo_dir.path().join("shell/zshrc")).unwrap(),
        "# my zshrc"
    );

    // Original location is now a symlink
    assert!(file_to_add.symlink_metadata().unwrap().file_type().is_symlink());

    // ditto.toml was updated
    let toml_content = std::fs::read_to_string(repo_dir.path().join("ditto.toml")).unwrap();
    assert!(toml_content.contains("zshrc"));
}

#[test]
fn add_fails_when_file_does_not_exist() {
    let repo_dir = tempfile::tempdir().unwrap();

    std::fs::write(
        repo_dir.path().join("ditto.toml"),
        "[settings]\nbackup_dir = \".ditto-backup\"\n\n[packages.shell]\nfiles = []\n",
    )
    .unwrap();

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("add")
        .arg("/nonexistent/file")
        .arg("--package")
        .arg("shell")
        .current_dir(repo_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn add_dry_run_does_not_move_file() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = home_tempdir();

    std::fs::write(
        repo_dir.path().join("ditto.toml"),
        "[settings]\nbackup_dir = \".ditto-backup\"\n\n[packages.shell]\nfiles = []\n",
    )
    .unwrap();

    let file_to_add = target_dir.path().join("zshrc");
    std::fs::write(&file_to_add, "# my zshrc").unwrap();

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("--dry-run")
        .arg("add")
        .arg(&file_to_add)
        .arg("--package")
        .arg("shell")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would move"));

    // File still in original location
    assert!(file_to_add.exists());
    assert!(!repo_dir.path().join("shell/zshrc").exists());
}
