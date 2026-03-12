pub mod cliargs;

use std::process::{Command, ExitCode};
use anyhow::{Context, Ok, Result, bail};
use clap::Parser;
use nix::unistd::geteuid;

use crate::cliargs::CliArgs;

const BASIC_PROMPT: &str = include_str!("../res/basic.txt");

fn ensure_git_repo() -> Result<()> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .context("Failed to execute git to check repository")?;

    if !output.status.success() {
        bail!("Not inside a git repository: git command failed");
    }

    let binding = String::from_utf8_lossy(&output.stdout);
    let is_inside = binding.trim();

    if is_inside != "true" {
        bail!("Not inside a git repository");
    }

    Ok(())
}

fn git_output(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git command")?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn main() -> Result<ExitCode> {
	if geteuid().is_root() {
		eprintln!("pccg should never be ran as root!");
		return Ok(ExitCode::FAILURE);
	}

    ensure_git_repo()?;

    let cli = CliArgs::parse();
    
    let status = git_output(&["status", "--porcelain"])
        .context("Failed to read git status")?;

    let diff = git_output(&["diff", "--staged"])
        .context("Failed to read staged diff")?;

    if status.is_empty() || diff.is_empty() {
        eprintln!("No staged changes detected.");
        return Ok(ExitCode::FAILURE);
    }

    let user_context = cli
        .context
        .as_deref()
        .map(|c| format!("\nOptional Context From User:\n{}\n", c))
        .unwrap_or_default();

    let prompt = BASIC_PROMPT
    .to_string()
    .replace("{context}", &user_context)
    .replace("{status}", &status)
    .replace("{diff}", &diff);
    print!("{prompt}");
    Ok(ExitCode::SUCCESS)
}
