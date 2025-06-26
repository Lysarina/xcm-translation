#!/bin/bash

root_dir=$(pwd)
variant="minmod"

find "$root_dir" -maxdepth 1 -type d -name 'rs-*' | while read -r rs_dir; do
  parent=$(dirname "$rs_dir")
  base=$(basename "$rs_dir")

  
    variant_dir="$parent/${variant}-${base}"

    # Remove old variant if it exists, then copy
    rm -rf "$variant_dir"
    cp -r "$rs_dir" "$variant_dir"
    echo "Created: $variant_dir"
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