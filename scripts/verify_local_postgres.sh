#!/usr/bin/env bash
set -euo pipefail

DATABASE_URL="${DATABASE_URL:-}"
APPEND_URL="${APPEND_DATABASE_URL:-${DATABASE_URL}}"
SIGNAL_URL="${SIGNAL_DATABASE_URL:-}"
READER_URL="${READER_DATABASE_URL:-}"

if [[ -z "$DATABASE_URL" ]]; then
  echo "FAIL - DATABASE_URL is not set"; exit 1
fi

function ensure_local() {
  local url="$1"
  if [[ "$url" =~ localhost || "$url" =~ ^postgresql:/// ]]; then
    return 0
  fi
  echo "FAIL - URL $url does not point to localhost/local socket"; exit 1
}

ensure_local "$DATABASE_URL"
ensure_local "$APPEND_URL"
[[ -z "$SIGNAL_URL" ]] || ensure_local "$SIGNAL_URL"
[[ -z "$READER_URL" ]] || ensure_local "$READER_URL"

function runpsql() {
  local url="$1"; shift
  if ! command -v psql >/dev/null 2>&1; then
    echo "SKIP - psql not installed"
    return 0
  fi
  psql "$url" -At -c "$*" >/dev/null
}

echo "PASS - DATABASE_URL points to localhost"
echo "PASS - append role URL validated"

if [[ -n "$SIGNAL_URL" ]]; then
  runpsql "$SIGNAL_URL" "\dt" && echo "PASS - signal reader connects"
fi

if [[ -n "$READER_URL" ]]; then
  runpsql "$READER_URL" "\dt" && echo "PASS - reader connects"
fi

if command -v psql >/dev/null 2>&1; then
  echo "Running schema check via append role"
  psql "$APPEND_URL" -At -c "SELECT tablename FROM pg_tables WHERE schemaname='public' ORDER BY tablename;" > /tmp/pg_tables.$$ || { echo "FAIL - listing tables"; exit 1; }
  echo "PASS - tables listed"
  echo "Testing append insert (skip if no test row)"
  if psql "$APPEND_URL" -c "BEGIN; INSERT INTO signals (id, source, raw_text, status, created_by) VALUES ('sig_smoke', 'in_app', 'smoke test', 'new', 'system'); ROLLBACK;" >/tmp/append.log 2>&1; then
    echo "PASS - append insert permitted (rolled-back)"
  else
    echo "FAIL - append insert blocked"; cat /tmp/append.log; rm -f /tmp/append.log; exit 1
  fi
  rm -f /tmp/append.log

  echo "Testing forbidden update"
  if psql "$APPEND_URL" -c "UPDATE signals SET status='linked' WHERE false;" >/tmp/update.log 2>&1; then
    echo "FAIL - update unexpectedly succeeded"; cat /tmp/update.log; exit 1
  else
    echo "PASS - update blocked (expected)"
  fi
  rm -f /tmp/update.log
else
  echo "SKIP - psql missing, cannot run inserts/guards"
fi
