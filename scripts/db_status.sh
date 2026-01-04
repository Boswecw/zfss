#!/usr/bin/env bash
set -euo pipefail

CONN="${DATABASE_URL:-}"
if [[ -z "$CONN" ]]; then
  echo "FAIL - DATABASE_URL must be set"
  exit 1
fi

HOST="$(hostname)"
TIMESTAMP="$(date +%Y%m%d_%H%M%S)"
LOG_DIR="$(cd "$(dirname "$0")/../receipts" && pwd)"
mkdir -p "$LOG_DIR"
LOG_FILE="$LOG_DIR/db_status_${TIMESTAMP}.log"
GIT_SHA="unknown"
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  GIT_SHA="$(git rev-parse --short HEAD)"
fi

{
  echo "timestamp=$TIMESTAMP"
  echo "host=$HOST"
  echo "git_sha=$GIT_SHA"
  echo "database_url=$CONN"
} > "$LOG_FILE"

FUNCTIONS=()

function fail {
  echo "FAIL - $1" | tee -a "$LOG_FILE"
  exit 1
}

if ! command -v psql >/dev/null 2>&1; then
  fail "psql is not installed"
fi

function check {
  local label="$1"
  shift
  if psql "$CONN" -At -c "$*" >/tmp/psql_check.$$ 2>&1; then
    echo "PASS - $label" | tee -a "$LOG_FILE"
  else
    echo "FAIL - $label" | tee -a "$LOG_FILE"
    cat /tmp/psql_check.$$
    rm -f /tmp/psql_check.$$
    exit 1
  fi
  rm -f /tmp/psql_check.$$
}

check "current database" "SELECT current_database()"
check "server address/port" "SELECT inet_server_addr(), inet_server_port()"
check "schema tables present" "SELECT tablename FROM pg_tables WHERE schemaname='public' ORDER BY tablename LIMIT 5"
check "append-only trigger exists" "SELECT tgname FROM pg_trigger WHERE tgname = 'zfss_forbid_mutation'"

echo "PASS - all status checks complete" | tee -a "$LOG_FILE"
