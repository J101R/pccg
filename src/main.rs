pub mod cli_args;
pub mod basic_context;
pub mod prompts;
pub mod git;

use std::{io::stdout, process::ExitCode};
use anyhow::{Context, Ok, Result};
use clap::Parser;
use minijinja::{Environment, Value};
use nix::unistd::geteuid;

use crate::{basic_context::BasicContext, cli_args::CliArgs};

fn main() -> Result<ExitCode> {
    if cfg!(unix) {
        if geteuid().is_root() {
            eprintln!("Do not run pccg as root");
            return Ok(ExitCode::FAILURE);
        }
    }

    git::ensure_git_repo()?;

    let cli = CliArgs::parse();
    
    let status = git::git_output(&["status", "--porcelain"])
        .context("Failed to read git status")?;

    let diff = git::git_output(&["diff", "--staged"])
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
        diff: diff,
        style: cli.style.to_string(),
    });
    tmpl.render_to_write(ctx, &mut stdout())
        .context("Failed to parse prompt")?;
    Ok(ExitCode::SUCCESS)
}
