#!/bin/bash
set -euo pipefail

HOOK_TYPE="commit-msg"
HOOK_PATH=".git/hooks/$HOOK_TYPE"

# Detect platform
UNAME="$(uname -s)"
ARCH="$(uname -m)"

case "$UNAME" in
  Darwin) OS="macos" ;;
  Linux)  OS="linux" ;;
  *) echo "[ERROR] Unsupported OS: $UNAME" && exit 1 ;;
esac

case "$ARCH" in
  x86_64 | amd64) ARCH_NAME="x86_64" ;;
  *) echo "[ERROR] Unsupported architecture: $ARCH" && exit 1 ;;
esac

# Construct expected binary name and path
BINARY_NAME="git-spellcheck-${OS}-${ARCH_NAME}"
BINARY_PATH="/usr/local/bin/${BINARY_NAME}"

# Validate binary exists
if [ ! -x "$BINARY_PATH" ]; then
  echo "[ERROR] Binary not found at: $BINARY_PATH"
  echo
  echo "You must download and install it from the latest release:"
  echo "➡️  https://github.com/kodelint/git-spellcheck/releases/latest"
  echo
  echo "After downloading:"
  echo "chmod +x $BINARY_NAME"
  echo "sudo mv $BINARY_NAME /usr/local/bin/"
  exit 1
fi

# Write commit-msg hook
cat <<EOF > "$HOOK_PATH"
#!/bin/sh
exec </dev/tty
exec "$BINARY_PATH" "\$1"
EOF

chmod +x "$HOOK_PATH"

echo "[INFO] Installed commit-msg hook at $HOOK_PATH"
echo "[INFO] Using binary: $BINARY_PATH"

