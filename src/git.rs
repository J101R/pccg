use std::process::Command;
use anyhow::{Context, Ok, Result, bail};

pub fn ensure_git_repo() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("Failed to execute git to check repository")?;

    if !output.status.success() {
        bail!("Not inside a git repository: (git command failed)")
    }

    let binding = String::from_utf8_lossy(&output.stdout);
    let is_inside = binding.trim();

    if is_inside != "true" {
        bail!("Not inside a git repository")
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

pub fn filter_status(status: String) -> String {
    status
        .lines()
        // Keep only lines where the first character is not ' ', '?' or '!'
        .filter(|line| {
            let mut chars = line.chars();
            match (chars.next(), chars.next(), chars.next()) {
                (Some(x), Some(_y), Some(space)) => {
                    x != ' ' && x != '?' && x != '!' && space == ' '
                },
                _ => false
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
