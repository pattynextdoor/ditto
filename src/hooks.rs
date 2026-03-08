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
