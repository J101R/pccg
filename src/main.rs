pub mod cli_args;
pub mod basic_context;
pub mod prompts;
pub mod git;

use std::{io::stdout, process::ExitCode};
use anyhow::{Context, Ok, Result, bail};
use clap::Parser;
use minijinja::{Environment, Value};
use nix::unistd::geteuid;

use crate::{basic_context::BasicContext, cli_args::CliArgs, git::filter_status};

fn main() -> Result<ExitCode> {
    if cfg!(unix) {
        if geteuid().is_root() {
            bail!("Do not run pccg as root")
        }
    }

    git::ensure_git_repo()?;

    let cli = CliArgs::parse();
    
    let mut status = git::git_output(&["status", "--porcelain"])
        .context("Failed to read git status")?;
    status = filter_status(status);
    let diff = git::git_output(&["diff", "--staged"])
        .context("Failed to read staged diff")?;

    if status.is_empty() || diff.is_empty() {
        bail!("No staged changes detected")
    }

    let env = Environment::new();
    let tmpl = env.template_from_str(prompts::BASIC_PROMPT)
        .context("Syntax error in basic prompt template")?;
    let ctx = Value::from_object(BasicContext {
        context: cli.context,
        status: status,
        diff: diff,
        style: cli.style.to_string(),
    });
    tmpl.render_captured_to(ctx, &mut stdout())
        .context("Failed to parse prompt")?;
    Ok(ExitCode::SUCCESS)
}
