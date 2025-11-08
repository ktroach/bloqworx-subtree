#!/usr/bin/env bash
set -euo pipefail

# —————————————————————————————————————————————————————————————
# Bloqworx WASM Bundler
# Builds the Rust crate and syncs its WebAssembly output
# with your Next.js app.
# —————————————————————————————————————————————————————————————

echo ""
echo "🦀🔗 Bloqworx WASM Bundler"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# —————————————————————————————————————————————————————————————
# Paths
# —————————————————————————————————————————————————————————————
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_DIR="$PROJECT_ROOT"
DEST_DIR="../bloqworx/public/lib/rust/subtree/pkg"

echo "📦 Crate directory:  $CRATE_DIR"
echo "📂 Target directory: $DEST_DIR"
echo ""

# —————————————————————————————————————————————————————————————
# Ensure dependencies
# —————————————————————————————————————————————————————————————
if ! command -v wasm-pack &> /dev/null; then
    echo "🧰 Installing wasm-pack..."
    cargo install wasm-pack
fi

if ! command -v wasm-bindgen &> /dev/null; then
    echo "🧰 Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# —————————————————————————————————————————————————————————————
# Build the optimized WASM bundle
# —————————————————————————————————————————————————————————————
echo "⚙️  Building optimized WebAssembly bundle..."
cd "$CRATE_DIR"
wasm-pack build --release --target web

# —————————————————————————————————————————————————————————————
# Sync to Next.js public directory
# —————————————————————————————————————————————————————————————
echo ""
echo "📁 Syncing files to Next.js public folder..."
mkdir -p "$DEST_DIR"

cp -v "$CRATE_DIR/pkg"/subtree.* "$DEST_DIR" 2>/dev/null || true
cp -v "$CRATE_DIR/pkg"/subtree_bg.* "$DEST_DIR" 2>/dev/null || true

# —————————————————————————————————————————————————————————————
# Summary
# —————————————————————————————————————————————————————————————
echo ""
echo "📋 Bundled files in Next.js public folder:"
ls -la "$DEST_DIR"

echo ""
echo "✅ Bundle complete!"
echo "🌐 Files ready at: public/lib/rust/subtree/pkg/"
echo "🚀 Serve your app with: npm run dev"
echo "💡 Access in browser: http://localhost:3000/wasm-test"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌟  Bloqworx WASM pipeline synced successfully!"
echo ""
