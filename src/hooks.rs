use anyhow::Result;

pub fn run_hooks(command: &str) -> Result<()> {
    let command_status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()?;

    if !command_status.success() {
        anyhow::bail!("Hook failed: {}", command);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_hook_returns_ok() {
        let result = run_hooks("true");
        assert!(result.is_ok());
    }

    #[test]
    fn failing_hook_returns_error() {
        let result = run_hooks("false");
        assert!(result.is_err());
    }

    #[test]
    fn hook_executes_shell_commands() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join("hook_ran");
        let cmd = format!("touch {}", marker.display());

        run_hooks(&cmd).unwrap();

        assert!(marker.exists());
    }
}
