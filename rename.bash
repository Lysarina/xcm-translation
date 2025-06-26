#!/bin/bash

set -e

root_dir=$(pwd)
variant="minmod"

find "$root_dir" -depth -type d -name "${variant}-rs-*" | while read -r src_dir; do
  parent=$(dirname "$src_dir")
  base=$(basename "$src_dir")

  # Remove prefix (e.g., "minmod-rs-" or "rustlike-rs-")
  new_base="rs-${base#${variant}-rs-}"
  dest_dir="$parent/$new_base"

  # Remove existing target, then copy
  rm -rf "$dest_dir"
  cp -r "$src_dir" "$dest_dir"
  echo "Copied: $src_dir → $dest_dir (overwriting if existed)"
done