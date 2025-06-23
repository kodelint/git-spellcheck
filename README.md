[![CI](https://github.com/kodelint/git-spellcheck/actions/workflows/release.yml/badge.svg)](https://github.com/kodelint/git-spellcheck/actions/workflows/release.yml)
[![GitHub release](https://img.shields.io/github/release/kodelint/git-spellcheck.svg)](https://github.com/kodelint/git-spellcheck/releases)
[![GitHub stars](https://img.shields.io/github/stars/kodelint/git-spellcheck.svg)](https://github.com/kodelint/git-spellcheck/stargazers)
[![Last commit](https://img.shields.io/github/last-commit/kodelint/git-spellcheck.svg)](https://github.com/kodelint/git-spellcheck/commits/main)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/kodelint/git-spellcheck/pulls)

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

## How it works
Assuming that `git-spellcheck` is already installed under `/usr/local/bin` and git repository already have `.git/hooks/commit-msg` enabled. If not run [install-hook.sh](./scripts/install-hook.sh)
### Using `git commit -m`
```bash
# Say a Git Repo
echo "Fun Fact" >> README.md
git add README.md
git commit -m "Initial committ to chck spelchekc" # Notice all the spelling mistakes

[ENTER]

git commit -m "Initial committ to chck spelchekc"
[SPELLCHECK] Found possible spelling mistakes:
  - committ
  - chck
  - spelchekc
[REPLACE] 'committ' - suggestions: commit, commits, com mitt, com-mitt, commit t, comity
Enter replacement (or press ENTER to skip): commit
[REPLACE] 'chck' - suggestions: tick, tuck, check, chick, chock, chuck, cock
Enter replacement (or press ENTER to skip): check
[REPLACE] 'spelchekc' - suggestions: spellcheck
Enter replacement (or press ENTER to skip): spellcheck
[master 7a198f8] Initial commit to check spellcheck
 1 file changed, 1 insertion(+)
```
##### Final commit, after spellcheck

```bash
[master 7a198f8] Initial commit to check spellcheck
 1 file changed, 1 insertion(+)
```

### Using `git commit`
```bash
echo "Fun Fact" >> README.md
git add README.md
git commit                   # open the default editor for commit message
git commit
[SPELLCHECK] Found possible spelling mistakes:
  - Aggin
  - tring
  - chekk
  - spellin
  - committ
  - defolt
[REPLACE] 'Aggin' - suggestions: Aggie, Ag gin, Ag-gin, Aging, Again, Nagging, Gagging, Tagging, Bagging
Enter replacement (or press ENTER to skip): Again
[REPLACE] 'tring' - suggestions: trig, ting, ring, string, tiring, taring, truing, trying, thing, bring, tying, wring, t ring, tripping
Enter replacement (or press ENTER to skip): trying
[REPLACE] 'chekk' - suggestions: cheek, check
Enter replacement (or press ENTER to skip): check
[REPLACE] 'spellin' - suggestions: spelling, spell in, spell-in, spellbind, spell, spline
Enter replacement (or press ENTER to skip): spelling
[REPLACE] 'committ' - suggestions: commit, commits, com mitt, com-mitt, commit t, comity
Enter replacement (or press ENTER to skip): commit
[REPLACE] 'defolt' - suggestions: defoliate
Enter replacement (or press ENTER to skip): default
[master e71379c] Again trying to check spelling on git commit, which opens the default editor
 1 file changed, 1 insertion(+)
```
##### Final commit, after spellcheck

```bash
[master e71379c] Again trying to check spelling on git commit, which opens the default editor
 1 file changed, 1 insertion(+)
```


## Installation

### Pre-built Binaries

Releases with pre-built binaries are available on GitHub [Releases](https://github.com/kodelint/git-spellcheck/releases). Download the binary for your platform and add it to your `$PATH`:

```bash
# macOS example
curl -LO https://github.com/kodelint/git-spellcheck/releases/latest/download/git-spellcheck-macos-x86_64
chmod +x git-spellcheck-macos-x86_64
sudo mv git-spellcheck-macos-x86_64 /usr/local/bin/git-spellcheck
```

---

### From Source

You need Rust installed ([rustup](https://rustup.rs/)).

```bash
git clone https://github.com/kodelint/git-spellcheck.git
cd git-spellcheck

# Download Hunspell dictionaries - Optional
mkdir -p src/assets
curl -fsSL -o src/assets/en_US.aff https://cgit.freedesktop.org/libreoffice/dictionaries/plain/en/en_US.aff
curl -fsSL -o src/assets/en_US.dic https://cgit.freedesktop.org/libreoffice/dictionaries/plain/en/en_US.dic

# Build release binary
cargo build --release

# If you see hunspell library issue 
LIBRARY_PATH=/opt/homebrew/lib cargo build --release (depending on where your homebrew libs are)
```
The binary will be at: `target/release/git-spellcheck`

## Usages

### Git Hook Setup 
- Run the script below
```bash
./scripts/install-hook.sh
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