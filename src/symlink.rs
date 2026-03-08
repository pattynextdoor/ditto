use std::path::Path;

use anyhow::Result;

pub fn create_symlink(src: &Path, target: &Path) -> Result<()> {
    std::os::unix::fs::symlink(src, target)?;
    Ok(())
}

pub fn is_symlink_to(path: &Path, expected: &Path) -> bool {
    if let Ok(link_target) = std::fs::read_link(path) {
        link_target == expected
    } else {
        false
    }
}

pub fn remove_symlink(path: &Path) -> Result<()> {
    std::fs::remove_file(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symlink_points_to_source_file() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.txt");
        let target = dir.path().join("link.txt");
        std::fs::write(&source, "hello").unwrap();

        create_symlink(&source, &target).unwrap();

        assert!(target.exists());
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "hello");
    }

    #[test]
    fn is_symlink_to_returns_true_for_correct_target() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.txt");
        let target = dir.path().join("link.txt");
        std::fs::write(&source, "hello").unwrap();
        create_symlink(&source, &target).unwrap();

        assert!(is_symlink_to(&target, &source));
    }

    #[test]
    fn is_symlink_to_returns_false_for_wrong_target() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.txt");
        let other = dir.path().join("other.txt");
        let target = dir.path().join("link.txt");
        std::fs::write(&source, "hello").unwrap();
        create_symlink(&source, &target).unwrap();

        assert!(!is_symlink_to(&target, &other));
    }

    #[test]
    fn is_symlink_to_returns_false_for_regular_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("regular.txt");
        let other = dir.path().join("other.txt");
        std::fs::write(&file, "hello").unwrap();

        assert!(!is_symlink_to(&file, &other));
    }

    #[test]
    fn is_symlink_to_returns_false_for_nonexistent_path() {
        assert!(!is_symlink_to(Path::new("/nonexistent"), Path::new("/other")));
    }

    #[test]
    fn remove_symlink_deletes_the_link() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.txt");
        let target = dir.path().join("link.txt");
        std::fs::write(&source, "hello").unwrap();
        create_symlink(&source, &target).unwrap();

        remove_symlink(&target).unwrap();

        assert!(!target.exists());
        assert!(source.exists()); // original file untouched
    }
}
