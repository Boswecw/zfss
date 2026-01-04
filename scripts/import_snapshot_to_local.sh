#!/usr/bin/env bash
set -euo pipefail

# Required: local authoritative Postgres URL (append role)
LOCAL_DATABASE_URL="${LOCAL_DATABASE_URL:?Point to the local Postgres append role (e.g., postgresql://dataforge_append:...@localhost:5432/dataforge)}"
EXPORT_DIR="${EXPORT_DIR:-$(pwd)/zfss_render_snapshot}"

readonly TABLE_ORDER=(
  "users"
  "issues"
  "issue_status_history"
  "signals"
  "signal_status_history"
  "attachments"
  "decisions"
  "artifacts"
  "responses"
  "response_approval_history"
  "audit_log"
)

echo "Preparing local DataForgeDB import from $EXPORT_DIR"

for table in "${TABLE_ORDER[@]}"; do
  file="$EXPORT_DIR/$table.csv"
  if [[ ! -f "$file" ]]; then
    echo "ERROR: missing snapshot file $file"
    exit 1
  fi
done

psql "$LOCAL_DATABASE_URL" -c "SET client_min_messages = WARNING; BEGIN; SET LOCAL synchronous_commit = OFF;"

for table in "${TABLE_ORDER[@]}"; do
  file="$EXPORT_DIR/$table.csv"
  echo "  ➜ loading $table from $(basename "$file")"
  psql "$LOCAL_DATABASE_URL" -c "\copy ${table} FROM '${file}' WITH CSV HEADER"
done

psql "$LOCAL_DATABASE_URL" <<'SQL'
-- Reset bigserial sequences owned by history/audit tables
SELECT pg_catalog.setval('issue_status_history_id_seq', COALESCE((SELECT MAX(id) FROM issue_status_history), 1), true);
SELECT pg_catalog.setval('signal_status_history_id_seq', COALESCE((SELECT MAX(id) FROM signal_status_history), 1), true);
SELECT pg_catalog.setval('response_approval_history_id_seq', COALESCE((SELECT MAX(id) FROM response_approval_history), 1), true);
SELECT pg_catalog.setval('audit_log_id_seq', COALESCE((SELECT MAX(id) FROM audit_log), 1), true);
COMMIT;
SQL

echo "Import complete. Run verification queries per zfss/docs/render_export_plan.md to ensure row counts and timestamps match the Render source."
