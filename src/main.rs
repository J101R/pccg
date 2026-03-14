pub mod cli_args;
pub mod basic_context;
pub mod prompts;

use std::{io::stdout, process::{Command, ExitCode}};
use anyhow::{Context, Ok, Result, bail};
use clap::Parser;
use minijinja::{Environment, Value};
use nix::unistd::geteuid;

use crate::{basic_context::BasicContext, cli_args::CliArgs};

fn ensure_git_repo() -> Result<()> {
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

fn git_output(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git command")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn main() -> Result<ExitCode> {
    if cfg!(unix) {
        if geteuid().is_root() {
            eprintln!("Do not run pccg as root");
            return Ok(ExitCode::FAILURE);
        }
    }

    ensure_git_repo()?;

    let cli = CliArgs::parse();
    
    let status = git_output(&["status", "--porcelain"])
        .context("Failed to read git status")?;

    let diff = git_output(&["diff", "--staged"])
        .context("Failed to read staged diff")?;

    if status.is_empty() || diff.is_empty() {
        eprintln!("No staged changes detected");
        return Ok(ExitCode::FAILURE);
    }

    let mut env = Environment::new();
    env.add_template("basic", prompts::BASIC_PROMPT)
        .context("Syntax error in basic prompt template")?;
    let tmpl = env.get_template("basic")
        .context("Failed to get basic template from the environment")?;
    let ctx = Value::from_object(BasicContext {
        context: cli.context,
        status: status,
        diff: diff
    });
    tmpl.render_to_write(ctx, &mut stdout())
        .context("Failed to parse prompt")?;
    Ok(ExitCode::SUCCESS)
}
