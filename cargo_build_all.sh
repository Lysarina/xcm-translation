#!/bin/bash

# cd /home/ehhjmou/xcm-translation/common/rs-util
# cargo +nightly build --release

# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-attr-node
# cargo +nightly build --release

# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-attr-path
# cargo +nightly build --release

# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-attr-tree
# cargo +nightly build --release

# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-attrpath
# cargo +nightly build --release
# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-core
# cargo +nightly build --release
# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-log
# cargo +nightly build --release
# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-log-attr-tree
# cargo +nightly build --release
# cd /home/ehhjmou/xcm-translation/libxcm/core/rs-xcm-addr
# cargo +nightly build --release

# cd /home/ehhjmou/xcm-translation/libxcm/tp/common/rs-active_fd
# cargo +nightly build --release
# cd /home/ehhjmou/xcm-translation/libxcm/tp/common/rs-xcm_tp
# cargo +nightly build --release

# cd /home/ehhjmou/xcm-translation/libxcm/tp/dns/rs-xcm-dns
# cargo +nightly build --release

# cd /home/ehhjmou/xcm-translation




set -e

root_dir=$(pwd)

# Find all directories whose basename starts with rs-
find . -type d \( -name target -prune \) -o -type d -name 'rs-*' -print | while read -r dir; do
    echo "Building Rust crate in $dir..."
    cd "$dir"
    # cargo clean
    cargo +nightly build --release
    cd "$root_dir"
done
