set -e

root_dir=$(pwd)

# Find all directories whose basename starts with rs-
find . -type d -name 'rs-*' | while read -r dir; do
    echo "Running clippy in $dir..."
    cd "$dir"
    cargo clippy
    cd "$root_dir"
done
