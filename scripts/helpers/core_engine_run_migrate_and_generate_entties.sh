#!/usr/bin/bash
SCRIPT_PATH="$(realpath "$BASH_SOURCE")"
SCRIPT_DIR="$(dirname "$(realpath "$BASH_SOURCE")")"
echo "Script path: $SCRIPT_PATH"
echo "Script directory: $SCRIPT_DIR"

sea-orm-cli migrate -d ./core-engine-db-migration/ down
sea-orm-cli migrate -d ./core-engine-db-migration/ up
sea-orm-cli generate entity -o ./core-engine-db/src/entities/