#!/bin/bash

set -e

root_dir=$(pwd)
variant="minmod"

echo "Setup for MinMod variant"

make clean

echo "Setting Makefile"
rm "Makefile.am"
cp -r "Makefile_rust.am" "Makefile.am"

# Change variant
echo "Changing Rust code variant"
find "$root_dir" -depth -type d -name "${variant}-rs-*" | while read -r src_dir; do
  parent=$(dirname "$src_dir")
  base=$(basename "$src_dir")

  new_base="rs-${base#${variant}-rs-}"
  dest_dir="$parent/$new_base"

  # Remove existing target, then copy
  rm -rf "$dest_dir"
  cp -r "$src_dir" "$dest_dir"
done

# Build Rust projects
find . -type d \( -name target -prune \) -o -type d -name 'rs-*' -print | while read -r dir; do
    echo "Building Rust crate in $dir..."
    cd "$dir"
    cargo +nightly build --release
    cd "$root_dir"
done