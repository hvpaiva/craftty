#!/bin/sh
set -e
./scripts/guard-release-files.sh --staged
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo nextest run --workspace
