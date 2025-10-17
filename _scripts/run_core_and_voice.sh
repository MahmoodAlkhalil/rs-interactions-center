#!/usr/bin/bash
SCRIPT_DIR="$(dirname "$(realpath "$BASH_SOURCE")")"
cd $SCRIPT_DIR/..
cargo run --bin core-engine &
cargo run --bin core-voice &