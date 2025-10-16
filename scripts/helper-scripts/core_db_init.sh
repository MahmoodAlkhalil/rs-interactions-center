#!/usr/bin/bash
SCRIPT_PATH="$(realpath "$BASH_SOURCE")"
SCRIPT_DIR="$(dirname "$(realpath "$BASH_SOURCE")")"

MIGRATION_DIR="$(realpath "$SCRIPT_DIR/../../core-engine-db/migration")"

cargo run --package core-engine-db-migration --bin main down
cargo run --package core-engine-db-migration --bin main up
# sea-orm-cli generate entity -o "$ENTITIES_REAL_PATH"