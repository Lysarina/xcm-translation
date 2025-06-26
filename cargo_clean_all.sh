set -e

root_dir=$(pwd)

# Find all directories whose basename starts with rs-
find . -type d \( -name target -prune \) -o -type d -name 'rs-*' -print | while read -r dir; do
    echo "Building Rust crate in $dir..."
    cd "$dir"
    cargo clean
    cd "$root_dir"
done