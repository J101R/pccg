use std::process::Command;
use anyhow::{Context, Ok, Result, bail};

pub fn ensure_git_repo() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("Failed to execute git to check repository")?;

    if !output.status.success() {
        bail!("Not inside a git repository: (git command failed)");
    }

    let binding = String::from_utf8_lossy(&output.stdout);
    let is_inside = binding.trim();

    if is_inside != "true" {
        bail!("Not inside a git repository");
    }

    Ok(())
}

pub fn git_output(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git command")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}