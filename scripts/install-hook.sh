#!/bin/bash

# Commit-msg hook location
HOOK_TYPE="commit-msg"
HOOK_PATH=".git/hooks/$HOOK_TYPE"

# Absolute path to compiled binary
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BINARY="$PROJECT_ROOT/target/release/git-spellcheck"

# Verify binary exists
if [ ! -f "$BINARY" ]; then
  echo "[ERROR] Build the tool first with: cargo build --release"
  exit 1
fi

# Write hook
cat <<EOF > "$HOOK_PATH"
#!/bin/sh
exec </dev/tty
exec "$BINARY" "\$1"
EOF

chmod +x "$HOOK_PATH"

echo "[INFO] Installed commit-msg hook at $HOOK_PATH"
