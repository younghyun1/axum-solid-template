#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${1:-$SCRIPT_DIR/be/.env}"

bash "$SCRIPT_DIR/be/setup-postgres.sh" "$ENV_FILE"
