#!/usr/bin/env bash
set -euo pipefail

branch_name=$(git symbolic-ref --short -q HEAD)
timestamp=$(date +"%Y-%m-%d %H:%M:%S")
current_datetime=$(date +"%m.%d.%Y.%H.%M.%S")

echo "🦀🚀 Shipping Rust crate: $branch_name"
echo "──────────────────────────────────────────────"

# Confirm clean workspace
if [ -n "$(git status --porcelain)" ]; then
    echo "📦 Staging and committing changes..."
    git add .
    git commit -am "Ship @ $current_datetime"
else
    echo "✅ No uncommitted changes."
fi

echo "📤 Pushing to branch '$branch_name'..."
git push --set-upstream origin "$branch_name"

echo ""
echo "✅ Shipped successfully at $timestamp"
