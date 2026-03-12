# Portable-Conventional-Commit-Generator

**Generate AI-ready prompts from staged Git changes.**

`pccg` is a lightweight CLI tool that reads your staged Git changes (status and diff) and formats them into a prompt ready to paste into an AI chatbot (like ChatGPT or Claude) to generate Conventional Commit messages.

## Features

* Detects staged Git changes automatically.
* Optionally includes user-provided context.
* Outputs a structured prompt ready for AI-assisted commit message generation.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/J101R/pccg.git
cd pccg
```

2. Build the project in release mode:

```bash
cargo build --release
```

3. Use the compiled binary:

```bash
# The binary is located in target/release/
./target/release/pccg > $HOME/Downloads/commit.txt
```

You can optionally move it to a directory in your PATH for easier use:

```bash
sudo mv target/release/pccg /usr/local/bin/
pccg > $HOME/Downloads/commit.txt
```

## Usage

```bash
# Generate a prompt from staged changes and save it to a file
pccg > $HOME/Downloads/commit.txt

# Include extra context
pccg --context "Refactored parser to improve error handling" > $HOME/Downloads/commit.txt
```

If there are no staged changes, the tool will print:

```text
No staged changes detected.
```

## License

This project is licensed under [GPL-3.0-or-later](LICENSE).
