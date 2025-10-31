#!/bin/bash

# Script to create a new release for the DECENTRALIZED-APP project

set -e  # Exit on any error

# Check if version argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <version> [commit-message]"
    echo "Example: $0 v1.2.3 \"Release version 1.2.3\""
    exit 1
fi

VERSION=$1
COMMIT_MESSAGE=${2:-"Release $VERSION"}

echo "Creating release $VERSION..."

# Check if tag already exists
if git rev-parse "$VERSION" >/dev/null 2>&1; then
    echo "Error: Tag $VERSION already exists!"
    exit 1
fi

# Update version in root Cargo.toml if it exists and differs
ROOT_CARGO="Cargo.toml"
if [ -f "$ROOT_CARGO" ]; then
    CURRENT_VERSION=$(grep "^version =" "$ROOT_CARGO" | head -n1 | cut -d '"' -f 2)
    if [ "$CURRENT_VERSION" != "${VERSION#v}" ]; then
        echo "Updating version in $ROOT_CARGO from $CURRENT_VERSION to ${VERSION#v}"
        sed -i "s/^version = \".*\"/version = \"${VERSION#v}\"/" "$ROOT_CARGO"
    fi
fi

# Update versions in all crate Cargo.toml files
find . -name "Cargo.toml" -not -path "./target/*" | while read -r file; do
    if grep -q "^version = " "$file"; then
        CURRENT_VERSION=$(grep "^version =" "$file" | head -n1 | cut -d '"' -f 2)
        if [ "$CURRENT_VERSION" != "${VERSION#v}" ]; then
            echo "Updating version in $file from $CURRENT_VERSION to ${VERSION#v}"
            sed -i "s/^version = \".*\"/version = \"${VERSION#v}\"/" "$file"
        fi
    fi
done

# Commit version changes if any
if ! git diff --quiet; then
    echo "Committing version updates..."
    git add .
    git commit -m "Bump version to ${VERSION#v}"
fi

# Create and push tag
echo "Creating and pushing tag $VERSION..."
git tag -a "$VERSION" -m "$COMMIT_MESSAGE"
git push origin "$VERSION"

echo "Release $VERSION created and pushed!"
echo "GitHub Actions will automatically create the release and publish packages."
echo "View progress at: https://github.com/attakdefand/DECENTRALIZED-APP/actions"