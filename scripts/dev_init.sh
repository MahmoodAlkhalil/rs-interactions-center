#!/usr/bin/bash
PG_PORT=22455
NATS_PORT=22456
NATS_WS_PORT=22457

while [[ $# -gt 0 ]]; do
    case $1 in
        --pg-pass)
            PG_PASS="$2"
            shift 2
            ;;
        --pg-data-dir)
            PG_DATA_DIR="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

if [[ -z "$PG_PASS" ]]; then
    echo "Error: --pg-pass option is required"
    exit 1
fi
echo "Password received: $PG_PASS"

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
if [[ -z "$PG_DATA_DIR" ]]; then
    echo "PG_DATA_DIR is empty or not set"
    podman run -d --replace --name rs-interactions-center-postgres -p $PG_PORT:5432 -e POSTGRES_PASSWORD="$PG_PASS" postgres:$POSTGRES_VERSION 2>/dev/null
else
    echo "PG_DATA_DIR is set to: $PG_DATA_DIR"
    podman run -d --replace --name rs-interactions-center-postgres -p $PG_PORT:5432 -e POSTGRES_PASSWORD="$PG_PASS" --volume $PG_DATA_DIR:/var/lib/postgresql postgres:$POSTGRES_VERSION 2>/dev/null
fi


NKEYS=$("${GO_APPS_PATH}/bin/nk" -gen user -pubout)
CORE_ENGINE_SEED=$(echo "$NKEYS" | head -n1)
CORE_ENGINE_PUB=$(echo "$NKEYS" | tail -n1)

mkdir $SCRIPT_DIR/helper-files/nats/dev-generated
cp -f $SCRIPT_DIR/helper-files/nats/skeletons/nats.conf.skel $SCRIPT_DIR/helper-files/nats/dev-generated/nats.conf
cp -f $SCRIPT_DIR/helper-files/nats/skeletons/users.conf.skel $SCRIPT_DIR/helper-files/nats/dev-generated/users.conf
sed -i "s/\"core_engine_pub_key\"/\"$CORE_ENGINE_PUB\"/g" $SCRIPT_DIR/helper-files/nats/dev-generated/users.conf

podman stop rs-interactions-center-nats > /dev/null 2>&1
podman container rm rs-interactions-center-nats > /dev/null 2>&1
podman run -d --replace --name rs-interactions-center-nats -p $NATS_PORT:4222 -p $NATS_WS_PORT:4223 -v $SCRIPT_DIR/helper-files/nats/dev-generated:/etc/nats/conf nats:$NATS_VERSION -c /etc/nats/conf/nats.conf

echo "Waiting for Postgres container to be ready and accepts connections"
until podman exec rs-interactions-center-postgres pg_isready -U postgres | grep -q "accepting connections"; do
    echo "postgres is not ready yet"
    sleep 2
done


CREATE_USER_QUERY="CREATE USER rsic WITH PASSWORD '$PG_PASS';"
UPDATE_USER_PASSWORD_QUERY="ALTER USER rsic WITH PASSWORD '$PG_PASS';"
ALLOW_USER_LOGIN_QUERY="ALTER USER rsic WITH LOGIN;"
CREATE_DATABASE_QUERY="CREATE DATABASE core OWNER rsic;"

echo "$CREATE_USER_QUERY"
podman exec rs-interactions-center-postgres psql -U postgres -c "$CREATE_USER_QUERY"
echo "$ALLOW_USER_LOGIN_QUERY"
podman exec rs-interactions-center-postgres psql -U postgres -c "$ALLOW_USER_LOGIN_QUERY"
echo "$UPDATE_USER_PASSWORD_QUERY"
podman exec rs-interactions-center-postgres psql -U postgres -c "$UPDATE_USER_PASSWORD_QUERY"
echo "$CREATE_DATABASE_QUERY"
podman exec rs-interactions-center-postgres psql -U postgres -c "$CREATE_DATABASE_QUERY"

echo "#auto generated env file data" > $SCRIPT_DIR/../.env
sed -i '/^RUST_LOG/d' $SCRIPT_DIR/../.env
echo "RUST_LOG=info" >> $SCRIPT_DIR/../.env

sed -i '/^DATABASE_URL/d' $SCRIPT_DIR/../.env
echo "DATABASE_URL=postgres://rsic:$PG_PASS@localhost:$PG_PORT/core" >> $SCRIPT_DIR/../.env

sed -i '/^NATS_URL/d' $SCRIPT_DIR/../.env
echo "NATS_URL=nats://localhost:$NATS_PORT" >> $SCRIPT_DIR/../.env

sed -i '/^NATS_SEED/d' $SCRIPT_DIR/../.env
echo "NATS_SEED=$CORE_ENGINE_SEED" >> $SCRIPT_DIR/../.env

sed -i '/^NATS_USERS_CONFIG_PATH/d' $SCRIPT_DIR/../.env
echo "NATS_USERS_CONFIG_PATH=$SCRIPT_DIR/helper-files/nats/dev-generated/users.conf" >> $SCRIPT_DIR/../.env

sed -i '/^NATS_NK_BIN_PATH/d' $SCRIPT_DIR/../.env
echo "NATS_NK_BIN_PATH=$GO_APPS_PATH/bin/nk" >> $SCRIPT_DIR/../.env


/usr/bin/bash $SCRIPT_DIR/helper-scripts/core_db_init.sh