#!/usr/bin/bash
sea-orm-cli migrate -d ./core-engine-db-migration/ down
sea-orm-cli migrate -d ./core-engine-db-migration/ up
sea-orm-cli generate entity -o ./core-engine/src/db/entities/