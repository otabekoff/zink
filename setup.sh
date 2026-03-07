#!/usr/bin/env bash
# setup.sh — Build and install Zink (interpreter + VS Code extension)
#
#   Usage:
#     ./setup.sh              Build & install everything
#     ./setup.sh lang         Build & install the Zink interpreter only
#     ./setup.sh extension    Build & install the VS Code extension only
#
#   The interpreter is installed via `cargo install --path lang`
#   which places the `zink` binary in ~/.cargo/bin (already on PATH).

set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"

step()  { printf '\n\033[36m⚡ %s\033[0m\n' "$1"; }
ok()    { printf '  \033[32m✓ %s\033[0m\n' "$1"; }
err()   { printf '  \033[31m✗ %s\033[0m\n' "$1"; }

# ── Interpreter ──────────────────────────────────────────────────────
install_lang() {
    step "Building Zink interpreter (release)..."
    pushd "$ROOT/lang" > /dev/null

    cargo build --release
    ok "Built lang/target/release/zink"

    step "Installing Zink to cargo bin..."
    cargo install --path . --force
    ok "zink installed to $(which zink 2>/dev/null || echo '~/.cargo/bin/zink')"
    echo "  Run \`zink --repl\` or \`zink <file.zink>\` from anywhere."

    popd > /dev/null
}

# ── VS Code Extension ───────────────────────────────────────────────
install_extension() {
    step "Building VS Code extension..."
    pushd "$ROOT/extension" > /dev/null

    if [ ! -d node_modules ]; then
        echo "  Installing npm dependencies..."
        npm install
    fi

    npm run compile
    ok "TypeScript compiled to out/"

    npm run package
    VSIX=$(ls -t *.vsix 2>/dev/null | head -1)
    if [ -z "$VSIX" ]; then
        err "No .vsix file found"
        exit 1
    fi
    ok "Packaged $VSIX ($(du -h "$VSIX" | cut -f1))"

    step "Installing extension into VS Code..."
    code --install-extension "$VSIX" --force
    ok "Extension installed — reload VS Code to activate."

    popd > /dev/null
}

# ── Main ─────────────────────────────────────────────────────────────
echo -e "\033[33m╔══════════════════════════════════════╗"
echo "║        Zink — Setup & Install        ║"
echo -e "╚══════════════════════════════════════╝\033[0m"

TARGET="${1:-all}"
case "$TARGET" in
    lang)      install_lang ;;
    extension) install_extension ;;
    all|*)     install_lang; install_extension ;;
esac

echo -e "\n\033[32m🎉 Done!\033[0m"
