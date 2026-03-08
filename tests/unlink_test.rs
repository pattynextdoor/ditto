#![allow(deprecated)]
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::{home_tempdir, setup_dotfiles_repo};

#[test]
fn unlink_removes_symlinks() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = home_tempdir();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    // Link first
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    assert!(
        target_file
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink()
    );

    // Unlink
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("unlink")
        .arg("--all")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("unlinked"));

    assert!(!target_file.exists());
    assert!(repo_dir.path().join("shell/.zshrc").exists());
}

#[test]
fn unlink_without_all_or_packages_warns_user() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("unlink")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No packages specified"));
}

#[test]
fn unlink_specific_package_only_removes_that_package() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = home_tempdir();
    let target_shell = target_dir.path().join(".zshrc");
    let target_git = target_dir.path().join(".gitconfig");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[
            ("shell".into(), target_shell.to_string_lossy().into()),
            ("git".into(), target_git.to_string_lossy().into()),
        ],
    );

    // Link all
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    // Unlink only shell
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("unlink")
        .arg("shell")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    assert!(!target_shell.exists());
    assert!(
        target_git
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn unlink_dry_run_does_not_remove_symlinks() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = home_tempdir();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    // Link first
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    // Dry run unlink
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("--dry-run")
        .arg("unlink")
        .arg("--all")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("would remove symlink"));

    assert!(
        target_file
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink()
    );
}
