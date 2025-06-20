# git-spellcheck

A Git commit message spell checker implemented in Rust using Hunspell dictionaries.  
It can be used as a Git commit-msg hook to prevent commits with spelling mistakes.

---

## Features

- Spellchecks commit messages before commits are finalized.
- Interactive inline suggestions for misspelled words.
- Supports `.spellignore` file to ignore specific words.
- Uses Hunspell dictionaries (bundled or configurable via environment variables).
- Cross-platform support (Linux & macOS).


---

## Installation

### Pre-built Binaries

Releases with pre-built binaries are available on GitHub [Releases](https://github.com/kodelint/git-spellcheck/releases).

Download the binary for your platform and add it to your PATH.

---

### From Source

You need Rust installed ([rustup](https://rustup.rs/)).

```bash
git clone https://github.com/yourusername/git-spellcheck.git
cd git-spellcheck

# Download Hunspell dictionaries
mkdir -p src/assets
curl -fsSL -o src/assets/en_US.aff https://cgit.freedesktop.org/libreoffice/dictionaries/plain/en/en_US.aff
curl -fsSL -o src/assets/en_US.dic https://cgit.freedesktop.org/libreoffice/dictionaries/plain/en/en_US.dic

# Build release binary
cargo build --release

# Binary will be at target/release/git-spellcheck
```

## Usages

### Git Hook Setup
- Add this to your Git repo’s `.git/hooks/commit-msg` file and make it executable:

```bash
#!/bin/sh

# Reconnect stdin to the terminal so Rust can prompt the user interactively
exec </dev/tty

exec git-spellcheck "$1"
```

- Make sure the hook file is executable:

```bash 
chmod +x .git/hooks/commit-msg
```

## Configuration
- Create a `.spellignore` file in your repository root to ignore specific words (one word per line).

## How It Works
- Reads your commit message. 
- Loads Hunspell dictionaries. 
- Checks each word for spelling. 
- Reports misspelled words and offers interactive suggestions. 
- Allows inline fixes before completing the commit. 
- Warns if mistakes remain and optionally aborts commit.