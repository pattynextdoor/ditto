#![allow(deprecated)]
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::{home_tempdir, setup_dotfiles_repo};

#[test]
fn link_creates_symlinks_for_all_packages() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("linked"));

    assert!(target_file.exists());
    assert!(
        target_file
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn link_specific_package_only_links_that_package() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_shell = target_dir.path().join(".zshrc");
    let target_git = target_dir.path().join(".gitconfig");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[
            ("shell".into(), target_shell.to_string_lossy().into()),
            ("git".into(), target_git.to_string_lossy().into()),
        ],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .arg("shell")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    assert!(
        target_shell
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert!(!target_git.exists());
}

#[test]
fn link_skips_already_linked_files() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    // Link once
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    // Link again -- should skip
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("already linked"));
}

#[test]
fn link_warns_on_conflict_without_force() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    // Create a conflicting file
    std::fs::write(&target_file, "existing content").unwrap();

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"));

    // Original file untouched
    assert_eq!(
        std::fs::read_to_string(&target_file).unwrap(),
        "existing content"
    );
}

#[test]
fn link_dry_run_does_not_create_symlinks() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("--dry-run")
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would create symlink"));

    assert!(!target_file.exists());
}

#[test]
fn link_force_overwrites_conflict_and_creates_backup() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = home_tempdir();
    let target_file = target_dir.path().join(".zshrc");

    // Create a conflicting file
    std::fs::write(&target_file, "existing content").unwrap();

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .arg("--force")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("linked"));

    // Target is now a symlink
    assert!(
        target_file
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink()
    );

    // Backup was created
    assert!(repo_dir.path().join(".ditto-backup").exists());
}

#[test]
fn link_force_dry_run_does_not_overwrite() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = home_tempdir();
    let target_file = target_dir.path().join(".zshrc");

    std::fs::write(&target_file, "existing content").unwrap();

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("--dry-run")
        .arg("link")
        .arg("--force")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would backup"));

    // Original file untouched
    assert_eq!(
        std::fs::read_to_string(&target_file).unwrap(),
        "existing content"
    );
}
