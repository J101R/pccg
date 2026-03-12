use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    rename_all = "kebab-case",
    version,
    about = "Generate an AI-ready prompt from staged git changes.",
    long_about = "Reads staged git changes (git status and git diff) and formats them \
into a prompt that can be pasted into a chat-based AI (ChatGPT, Claude, etc.) \
to generate a commit message."
)]
pub struct CliArgs {
    #[arg(
        short,
        long,
        value_name = "TEXT",
        help = "Extra context about the changes to include in the prompt.",
        long_help = "Additional context describing the purpose or intent of the changes.

This text will be included in the generated prompt alongside the staged
git status and diff before you paste it into an AI chatbot.

Example:
    --context \"Refactored parser to improve error handling\""
    )]
    pub(crate) context: Option<String>,
}
