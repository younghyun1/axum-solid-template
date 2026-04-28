#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

ENV_FILE="${1:-$SCRIPT_DIR/.env}"
if [[ ! -f "$ENV_FILE" ]]; then
  echo "missing env file: $ENV_FILE"
  echo "usage: $0 [path-to-env]"
  exit 1
fi

set -o allexport
# shellcheck disable=SC1091
source "$ENV_FILE"
set +o allexport

# Support both DB_* and POSTGRES_* style keys
DB_NAME_RESOLVED="${DB_NAME:-${POSTGRES_DB:-${PGDATABASE:-}}}"
DB_USER_RESOLVED="${DB_USER:-${POSTGRES_USER:-${PGUSER:-}}}"
DB_PASSWORD_RESOLVED="${DB_PASSWORD:-${POSTGRES_PASSWORD:-${PGPASSWORD:-}}}"
DB_HOST_RESOLVED="${DB_HOST:-${POSTGRES_HOST:-localhost}}"
DB_PORT_RESOLVED="${DB_PORT:-${POSTGRES_PORT:-5432}}"
DB_SCHEMA_RESOLVED="${DB_SCHEMA:-${POSTGRES_SCHEMA:-public}}"

: "${DB_NAME_RESOLVED?one of DB_NAME/POSTGRES_DB/PGDATABASE must be set in $ENV_FILE}"
: "${DB_USER_RESOLVED?one of DB_USER/POSTGRES_USER/PGUSER must be set in $ENV_FILE}"
: "${DB_PASSWORD_RESOLVED?one of DB_PASSWORD/POSTGRES_PASSWORD/PGPASSWORD must be set in $ENV_FILE}"

export DB_NAME="$DB_NAME_RESOLVED"
export DB_USER="$DB_USER_RESOLVED"
export DB_PASSWORD="$DB_PASSWORD_RESOLVED"
export DB_HOST="${DB_HOST_RESOLVED}"
export DB_PORT="${DB_PORT_RESOLVED}"
export DB_SCHEMA="${DB_SCHEMA_RESOLVED}"

PSQL_COMMON=(-v ON_ERROR_STOP=1 -v db_user="$DB_USER" -v db_pass="$DB_PASSWORD" -v db_name="$DB_NAME" -h "$DB_HOST" -p "$DB_PORT")

sudo -iu postgres psql "${PSQL_COMMON[@]}" <<'SQL'
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = :'db_user') THEN
    EXECUTE format('CREATE ROLE %I LOGIN PASSWORD %L', :'db_user', :'db_pass');
  END IF;
END $$;

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_database WHERE datname = :'db_name') THEN
    EXECUTE format('CREATE DATABASE %I OWNER %I', :'db_name', :'db_user');
  END IF;
END $$;
SQL

sudo -iu postgres psql "${PSQL_COMMON[@]}" -v db_schema="$DB_SCHEMA" -d "$DB_NAME" <<'SQL'
GRANT ALL PRIVILEGES ON SCHEMA :db_schema TO :db_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA :db_schema TO :db_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA :db_schema TO :db_user;
ALTER DEFAULT PRIVILEGES IN SCHEMA :db_schema GRANT ALL ON TABLES TO :db_user;
REVOKE ALL ON DATABASE :db_name FROM PUBLIC;
SQL

echo "PostgreSQL user '$DB_USER' and database '$DB_NAME' are ready."
echo "Environment: $ENV_FILE"
