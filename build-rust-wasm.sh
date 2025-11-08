#!/usr/bin/env bash
set -euo pipefail

# —————————————————————————————————————————————————————————————
# Locals
# —————————————————————————————————————————————————————————————
# This script should live anywhere inside or beside the project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_DIR="$PROJECT_ROOT"  # Rust crate root (bloqworx-subtree)

# —————————————————————————————————————————————————————————————
# Ensure wasm-pack is installed
# —————————————————————————————————————————————————————————————
if ! command -v wasm-pack &> /dev/null; then
    echo "ℹ️  wasm-pack not found; installing via cargo..."
    cargo install wasm-pack
fi

# (Optional) Ensure wasm-bindgen CLI is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "ℹ️  wasm-bindgen CLI not found; installing via cargo..."
    cargo install wasm-bindgen-cli
fi

# —————————————————————————————————————————————————————————————
# Crate setup
# —————————————————————————————————————————————————————————————
cd "$CRATE_DIR"

# Patch Cargo.toml to add cdylib crate-type if needed
if ! grep -q 'crate-type' Cargo.toml; then
    echo "📑  Adding cdylib crate-type to Cargo.toml..."
    cat << 'EOF' >> Cargo.toml

[lib]
crate-type = ["cdylib","rlib"]
EOF
fi

echo "📦  Fetching Rust dependencies..."
cargo fetch

echo "🧹  Cleaning previous build artifacts..."
cargo clean

echo "🔨  Building the WASM bundle (release, web target)..."
wasm-pack build --release --target web

# —————————————————————————————————————————————————————————————
# (Optional) Copy pkg/ into your Next.js app
# —————————————————————————————————————————————————————————————
# Example: copy to your Next.js app’s public folder
# JS_APP_PATH="../bloqworx/public/wasm"
# mkdir -p "$JS_APP_PATH"
# cp -r pkg/* "$JS_APP_PATH"

echo "✅  Build complete! pkg/ is in:"
echo "   $CRATE_DIR/pkg"
