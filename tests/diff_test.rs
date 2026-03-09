#![allow(deprecated)]
mod common;

use assert_cmd::Command;
use predicates::prelude::*;

use common::setup_dotfiles_repo;

#[test]
fn diff_shows_no_changes_when_files_match() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    // Link so target is a symlink to source (identical content)
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("link")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("diff")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("in sync"));
}

#[test]
fn diff_shows_changes_when_target_differs_from_source() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    // Write a different file at the target location (not a symlink)
    std::fs::write(&target_file, "modified content").unwrap();

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("diff")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("[diff]"))
        .stdout(predicate::str::contains("shell"));
}

#[test]
fn diff_specific_package_only_diffs_that_package() {
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

    // Make both targets differ from source
    std::fs::write(&target_shell, "modified shell").unwrap();
    std::fs::write(&target_git, "modified git").unwrap();

    // Only diff shell
    let output = Command::cargo_bin("ditto")
        .unwrap()
        .arg("diff")
        .arg("shell")
        .current_dir(repo_dir.path())
        .assert()
        .success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("shell"));
    assert!(!stdout.contains("git"));
}

#[test]
fn diff_skips_files_where_target_does_not_exist() {
    let repo_dir = tempfile::tempdir().unwrap();
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");

    setup_dotfiles_repo(
        repo_dir.path(),
        &[("shell".into(), target_file.to_string_lossy().into())],
    );

    // Don't create the target file -- it doesn't exist
    Command::cargo_bin("ditto")
        .unwrap()
        .arg("diff")
        .current_dir(repo_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("in sync"));
}
