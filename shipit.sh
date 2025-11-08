#!/usr/bin/env bash
set -euo pipefail

branch_name=$(git symbolic-ref --short -q HEAD)
timestamp=$(date +"%Y-%m-%d %H:%M:%S")
current_datetime=$(date +"%m.%d.%Y.%H.%M.%S")

echo ""
echo "🦀🚀 Shipping Rust crate: $branch_name"
echo "──────────────────────────────────────────────"

# —————————————————————————————————————————————————————————————
# Step 1: Build fresh WASM bundle before shipping
# —————————————————————————————————————————————————————————————
if [ -f "./bundle-wasm.sh" ]; then
    echo "🧱 Building fresh WASM bundle..."
    ./bundle-wasm.sh
else
    echo "⚠️  Skipping WASM build (bundle-wasm.sh not found)"
fi

# —————————————————————————————————————————————————————————————
# Step 2: Stage & commit any changes
# —————————————————————————————————————————————————————————————
if [ -n "$(git status --porcelain)" ]; then
    echo "📦 Staging and committing changes..."
    git add .
    git commit -am "Ship @ $current_datetime"
else
    echo "✅ No uncommitted changes."
fi

# —————————————————————————————————————————————————————————————
# Step 3: Push changes to current branch
# —————————————————————————————————————————————————————————————
echo "📤 Pushing to branch '$branch_name'..."
git push --set-upstream origin "$branch_name"

# —————————————————————————————————————————————————————————————
# Step 4: Auto-tag the release
# —————————————————————————————————————————————————————————————
version_tag="v$(date +'%y.%m.%d.%H%M')"
echo "🏷️  Creating tag: $version_tag"
git tag -a "$version_tag" -m "Automated release $version_tag"
git push origin "$version_tag"

# —————————————————————————————————————————————————————————————
# Step 5: Summary
# —————————————————————————————————————————————————————————————
echo ""
echo "✅ Shipped successfully at $timestamp"
echo "🏷️  Tagged release as: $version_tag"
echo "🌐  Repo: https://github.com/ktroach/bloqworx-subtree"
echo "──────────────────────────────────────────────"
