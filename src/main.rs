pub mod cliargs;

use std::{process::{Command, ExitCode}, sync::Arc};
use anyhow::{Context, Ok, Result, bail};
use clap::Parser;
use minijinja::{Environment, Value, value::Object};
use nix::unistd::geteuid;

use crate::cliargs::CliArgs;

const BASIC_PROMPT: &str = include_str!("../res/basic.txt");

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

#[derive(Debug)]
pub struct BasicContext {
    context: Option<String>,
    status: String,
    diff: String,
}

impl Object for BasicContext {
    fn get_value(self: &Arc<Self>, field: &Value) -> Option<Value> {
        match field.as_str()? {
            "context" => Some(Value::from(self.context.clone())),
            "status" => Some(Value::from(self.status.clone())),
            "diff" => Some(Value::from(self.diff.clone())),
            _ => None
        }
    }
}

fn main() -> Result<ExitCode> {
	if geteuid().is_root() {
		eprintln!("Do not run pccg as root");
		return Ok(ExitCode::FAILURE);
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
    env.add_template("basic", BASIC_PROMPT)?;
    let tmpl = env.get_template("basic")?;
    let ctx = Value::from_object(BasicContext {
        context: cli.context,
        status: status,
        diff: diff
    });
    let prompt = tmpl.render(ctx)?;
    print!("{prompt}");
    Ok(ExitCode::SUCCESS)
}
