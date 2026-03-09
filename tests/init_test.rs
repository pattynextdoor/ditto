#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;

/// Create a bare git repo that looks like a dotfiles repo with a ditto.toml.
fn setup_bare_repo(target_path: &str) -> tempfile::TempDir {
    let work_dir = tempfile::tempdir().unwrap();
    let bare_dir = tempfile::tempdir().unwrap();

    // Init a bare repo
    let out = std::process::Command::new("git")
        .args(["init", "--bare"])
        .arg(bare_dir.path())
        .output()
        .unwrap();
    assert!(out.status.success(), "git init --bare failed");

    // Create a working clone
    let work = work_dir.path().join("work");
    let out = std::process::Command::new("git")
        .args(["clone"])
        .arg(bare_dir.path())
        .arg(&work)
        .output()
        .unwrap();
    assert!(out.status.success(), "git clone failed");

    // Configure git user (CI runners may not have this set)
    std::process::Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&work)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&work)
        .output()
        .unwrap();

    let toml_content = format!(
        "[settings]\nbackup_dir = \".ditto-backup\"\n\n\
         [packages.shell]\nfiles = [\n  {{ src = \"shell/.zshrc\", target = \"{}\" }},\n]\n",
        target_path
    );
    std::fs::write(work.join("ditto.toml"), toml_content).unwrap();
    std::fs::create_dir_all(work.join("shell")).unwrap();
    std::fs::write(work.join("shell/.zshrc"), "# managed by ditto").unwrap();

    let out = std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(&work)
        .output()
        .unwrap();
    assert!(out.status.success(), "git add failed");

    let out = std::process::Command::new("git")
        .args(["commit", "-m", "init"])
        .current_dir(&work)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let out = std::process::Command::new("git")
        .args(["push"])
        .current_dir(&work)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git push failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    bare_dir
}

#[test]
fn init_clones_repo_and_links() {
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");
    let clone_dest = tempfile::tempdir().unwrap();
    let clone_path = clone_dest.path().join("dotfiles");

    let bare_repo = setup_bare_repo(&target_file.to_string_lossy());

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("init")
        .arg(bare_repo.path().to_str().unwrap())
        .arg("--path")
        .arg(&clone_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("linked"))
        .stdout(predicate::str::contains("transformed"));

    // Repo was cloned
    assert!(clone_path.join("ditto.toml").exists());

    // Symlink was created
    assert!(
        target_file
            .symlink_metadata()
            .unwrap()
            .file_type()
            .is_symlink()
    );
}

#[test]
fn init_fails_with_invalid_repo_url() {
    let clone_dest = tempfile::tempdir().unwrap();
    let clone_path = clone_dest.path().join("dotfiles");

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("init")
        .arg("/nonexistent/repo.git")
        .arg("--path")
        .arg(&clone_path)
        .assert()
        .failure();
}

#[test]
fn init_dry_run_does_not_create_symlinks() {
    let target_dir = tempfile::tempdir().unwrap();
    let target_file = target_dir.path().join(".zshrc");
    let clone_dest = tempfile::tempdir().unwrap();
    let clone_path = clone_dest.path().join("dotfiles");

    let bare_repo = setup_bare_repo(&target_file.to_string_lossy());

    Command::cargo_bin("ditto")
        .unwrap()
        .arg("--dry-run")
        .arg("init")
        .arg(bare_repo.path().to_str().unwrap())
        .arg("--path")
        .arg(&clone_path)
        .assert()
        .success();

    // Repo was cloned (dry-run only affects linking)
    assert!(clone_path.join("ditto.toml").exists());

    // Symlinks should not be created in dry-run
    assert!(!target_file.exists());
}
