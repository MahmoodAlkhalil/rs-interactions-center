#!/usr/bin/bash
SCRIPT_PATH="$(realpath "$BASH_SOURCE")"
SCRIPT_DIR="$(dirname "$(realpath "$BASH_SOURCE")")"

MIGRATION_DIR="$(realpath "$SCRIPT_DIR/../../core-engine-db/migration")"

sea-orm-cli migrate -d "$MIGRATION_DIR" down
sea-orm-cli migrate -d "$MIGRATION_DIR" up
# sea-orm-cli generate entity -o "$ENTITIES_REAL_PATH"