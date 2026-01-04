#!/usr/bin/env bash
set -euo pipefail

SQL_DIR="$(cd "$(dirname "$0")/../migrations" && pwd)"
FILES=("001_initial_schema.sql" "002_append_only_enforcement.sql" "003_signal_link_events.sql")

CONN="${DATAFORGE_OWNER_URL:-${DATABASE_URL:-}}"
if [[ -z "$CONN" ]]; then
  echo "DATAFORGE_OWNER_URL or DATABASE_URL must be set"
  exit 1
fi

RECEIPTS_DIR="$(cd "$(dirname "$0")/../receipts" && pwd)"
mkdir -p "$RECEIPTS_DIR"
TIMESTAMP="$(date +"%Y%m%d_%H%M%S")"
HOST="$(hostname)"
GIT_SHA="unknown"
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  GIT_SHA="$(git rev-parse --short HEAD)"
fi

LOG_FILE="$RECEIPTS_DIR/schema_apply_${TIMESTAMP}.log"
{
  printf "start_time=%s\nhost=%s\npg_url=%s\n" "$TIMESTAMP" "$HOST" "$CONN"
  printf "git_sha=%s\n" "$GIT_SHA"
  printf "files=%s\n" "${FILES[*]}"
} > "$LOG_FILE"

STATUS="PASS"
for file in "${FILES[@]}"; do
  SQL_PATH="$SQL_DIR/$file"
  if [[ ! -f "$SQL_PATH" ]]; then
    echo "Missing migration file: $SQL_PATH"
    STATUS="FAIL"
    break
  fi
  if ! PGPASSWORD="${PGPASSWORD:-}" psql "$CONN" --set ON_ERROR_STOP=1 -f "$SQL_PATH"; then
    STATUS="FAIL"
    break
  fi
done

echo "status=$STATUS" >> "$LOG_FILE"
if [[ "$STATUS" == "PASS" ]]; then
  echo "Schema applied successfully" >> "$LOG_FILE"
else
  echo "Schema apply failed" >> "$LOG_FILE"
  exit 1
fi
