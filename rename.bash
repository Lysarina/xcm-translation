#!/bin/bash

set -e

root_dir=$(pwd)
variant="rustlike"

find . -type d \( -name target -prune \) -o -type d -name 'rs-*' -print | while read -r rs_dir; do

#find "$root_dir" -maxdepth 1 -type d -name 'rs-*' | while read -r rs_dir; do
  parent=$(dirname "$rs_dir")
  base=$(basename "$rs_dir")
  echo $parent
  echo $base

  
    variant_dir="$parent/${variant}-${base}"
    echo $variant_dir

    # Remove old variant if it exists, then copy
    
    cp -r "$rs_dir" "$variant_dir"
    echo "Created: $variant_dir"
    cd "$root_dir"
done

# find "$root_dir" -depth -type d -name "${variant}-rs-*" | while read -r src_dir; do
#   parent=$(dirname "$src_dir")
#   base=$(basename "$src_dir")

#   # Remove prefix (e.g., "minmod-rs-" or "rustlike-rs-")
#   new_base="rs-${base#${variant}-rs-}"
#   dest_dir="$parent/$new_base"

#   # Remove existing target, then copy
#   rm -rf "$dest_dir"
#   cp -r "$src_dir" "$dest_dir"
#   echo "Copied: $src_dir → $dest_dir (overwriting if existed)"
# done