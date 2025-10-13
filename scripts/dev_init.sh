#!/bin/bash

while [[ $# -gt 0 ]]; do
    case $1 in
        --pgpass)
            PGPASS="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [[ -z "$PGPASS" ]]; then
    echo "Error: --pgpass option is required"
    exit 1
fi
echo "Password received: $PGPASS"

SCRIPT_PATH="$(realpath "$BASH_SOURCE")"
SCRIPT_DIR="$(dirname "$(realpath "$BASH_SOURCE")")"
NATS_VERSION="2.12.0"
POSTGRES_VERSION="18.0"
echo "Script path: $SCRIPT_PATH"
echo "Script directory: $SCRIPT_DIR"

echo "Checking for podman..."
if ! command -v podman >/dev/null 2>&1; then
  echo "Error: podman is not installed. Please install podman." >&2
  exit 1
fi

echo "Checking for Go..."
if ! command -v go >/dev/null 2>&1; then
  echo "Error: Go is not installed. Please install Go." >&2
  exit 1
fi
GO_APPS_PATH="$(go env GOPATH)"

go install github.com/nats-io/nkeys/nk@latest 

if podman image exists "postgres:${POSTGRES_VERSION}"; then
    echo "Postgres image docker.io/postgres:$POSTGRES_VERSION already exists"
else
    echo "Pulling NATS image docker.io/postgres:$POSTGRES_VERSION"
    podman pull docker.io/postgres:$POSTGRES_VERSION
fi

if podman image exists "nats:${NATS_VERSION}"; then
    echo "NATS image docker.io/nats:$NATS_VERSION already exists"
else
    echo "Pulling Postgresql image docker.io/nats:$NATS_VERSION"
    podman pull docker.io/nats:$NATS_VERSION
fi


podman stop rs-interactions-center-postgres > /dev/null 2>&1
podman container rm rs-interactions-center-postgres > /dev/null 2>&1
podman run -d --replace --name rs-interactions-center-postgres -p 5432:5432 -e POSTGRES_PASSWORD="$PGPASS" postgres:$POSTGRES_VERSION 2>/dev/null

NKEYS=$("${GO_APPS_PATH}/bin/nk" -gen user -pubout)
CORE_ENGINE_SEED=$(echo "$NKEYS" | head -n1)
CORE_ENGINE_PUB=$(echo "$NKEYS" | tail -n1)

cp -f $SCRIPT_DIR/helper-files/nats/skeletons/nats.conf.skel $SCRIPT_DIR/helper-files/nats/dev-generated/nats.conf
cp -f $SCRIPT_DIR/helper-files/nats/skeletons/users.conf.skel $SCRIPT_DIR/helper-files/nats/dev-generated/users.conf
sed -i "s/\"core_engine_pub_key\"/\"$CORE_ENGINE_PUB\"/g" $SCRIPT_DIR/helper-files/nats/dev-generated/users.conf

podman stop rs-interactions-center-nats > /dev/null 2>&1
podman container rm rs-interactions-center-nats > /dev/null 2>&1
podman run -d --replace --name rs-interactions-center-nats -p 4222:4222 -p 4223:4223 -v $SCRIPT_DIR/helper-files/nats/dev-generated:/etc/nats/conf nats:$NATS_VERSION -c /etc/nats/conf/nats.conf

echo "Waiting for Postgres container to be ready and accepts connections"
until podman exec rs-interactions-center-postgres pg_isready -U postgres | grep -q "accepting connections"; do
    echo "postgres is not ready yet"
    sleep 2
done

echo "Creating rsic user"
podman exec rs-interactions-center-postgres psql -U postgres -c "CREATE USER rsic WITH PASSWORD '$PGPASS';"
echo "Creating core database"
podman exec rs-interactions-center-postgres psql -U postgres -c "CREATE DATABASE core OWNER rsic;"
echo "allowing rsic user remote login"
podman exec rs-interactions-center-postgres psql -U postgres -c "ALTER USER rsic WITH LOGIN;"

