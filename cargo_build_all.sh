set -e

root_dir=$(pwd)

find . -type d \( -name target -prune \) -o -type d -name 'rs-*' -print | while read -r dir; do
    echo "Building Rust crate in $dir..."
    cd "$dir"
    cargo +nightly build --release
    cd "$root_dir"
done
