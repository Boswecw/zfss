#!/usr/bin/env bash
set -euo pipefail

# Render connection string (from the dashboard, e.g. `postgres://forge_user:<pw>@forge-db.<region>.render.com:5432/forge`)
RENDER_DATABASE_URL="${RENDER_DATABASE_URL:?Set RENDER_DATABASE_URL to the Render Postgres connection string}"

EXPORT_DIR="${EXPORT_DIR:-$(pwd)/zfss_render_snapshot}"
mkdir -p "$EXPORT_DIR"

declare -a TABLE_QUERIES=(
    # Dependency-safe order: users → issues → histories → signals → attachments/decisions/etc.
    "users|SELECT * FROM users ORDER BY created_at, id"
    "issues|SELECT * FROM issues ORDER BY created_at, id"
    "issue_status_history|SELECT * FROM issue_status_history ORDER BY changed_at, id"
    "signals|SELECT * FROM signals ORDER BY created_at, id"
    "signal_status_history|SELECT * FROM signal_status_history ORDER BY changed_at, id"
    "attachments|SELECT * FROM attachments ORDER BY created_at, id"
    "decisions|SELECT * FROM decisions ORDER BY decided_at, id"
    "artifacts|SELECT * FROM artifacts ORDER BY created_at, id"
    "responses|SELECT * FROM responses ORDER BY drafted_at, id"
    "response_approval_history|SELECT * FROM response_approval_history ORDER BY changed_at, id"
    "audit_log|SELECT * FROM audit_log ORDER BY created_at, id"
)

echo "Exporting Render DataForgeDB snapshot to $EXPORT_DIR"

for entry in "${TABLE_QUERIES[@]}"; do
    table="${entry%%|*}"
    query="${entry#*|}"
    dest="$EXPORT_DIR/${table}.csv"

    echo "  - exporting $table → $dest (ordered query)"
    psql "$RENDER_DATABASE_URL" -c "\copy (${query}) TO '${dest}' WITH CSV HEADER"
done

echo "Snapshot complete. Please keep ${EXPORT_DIR} read-only and compress for transport."
