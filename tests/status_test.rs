#![allow(deprecated)]
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::setup_dotfiles_repo;

#[test]
fn status_shows_not_linked_when_no_symlinks_exist() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("status")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("not linked"));
}

#[test]
fn status_shows_linked_after_linking() {
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
        .success();

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("status")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("linked"));
}

#[test]
fn status_shows_conflict_when_non_symlink_exists() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    std::fs::write(&target_file, "existing content").unwrap();

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("status")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("conflict"));
}
