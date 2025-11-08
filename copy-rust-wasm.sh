#!/bin/bash
set -euo pipefail

# Copy the Rust WASM files to the Next.js public directory
# Run this from: C:\Users\MXZ USER\Developments\apps\bloqworx

echo "🤖🦀 Copying Rust WASM Subtree files to public directory..."

# Source directory (the pkg folder inside your Rust project)
SOURCE_DIR="../bloqworx-subtree/pkg"

# Destination directory (inside your Next.js app)
DEST_DIR="public/lib/rust/subtree/pkg"

# Ensure source exists
if [ ! -d "$SOURCE_DIR" ]; then
    echo "❌ Error: Source directory '$SOURCE_DIR' not found!"
    echo "Make sure you've built the Rust project with wasm-pack first."
    echo "Example: cd ../bloqworx-subtree && ./build-rust-wasm.sh"
    exit 1
fi

# Create destination folder if missing
if [ ! -d "$DEST_DIR" ]; then
    echo "📁 Creating destination directory: $DEST_DIR"
    mkdir -p "$DEST_DIR"
fi

echo "📂 Copying from: $SOURCE_DIR"
echo "📂 Copying to:   $DEST_DIR"

# Copy core WASM and JS files
cp -v "$SOURCE_DIR"/subtree.* "$DEST_DIR" 2>/dev/null || true
cp -v "$SOURCE_DIR"/subtree_bg.* "$DEST_DIR" 2>/dev/null || true

echo ""
echo "📋🦀 Files now in public directory:"
ls -la "$DEST_DIR"

echo ""
echo "🚀🦀 WASM files ready for Next.js!"
echo "💡🦀 You can now test at: http://localhost:3000/wasm-test"
