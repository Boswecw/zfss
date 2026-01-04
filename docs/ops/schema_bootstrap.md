# Schema Bootstrap Guide

## Purpose

This documents how to initialize the local `dataforge` PostgreSQL schema using the existing migrations that already define the required tables, triggers, and guardrails (`zfss/migrations/001_initial_schema.sql`, `002_append_only_enforcement.sql`, `003_signal_link_events.sql`).

## Prerequisites

- `psql` installed and callable.
- Local PostgreSQL service running.
- `dataforge` database exists and is owned by `dataforge_owner`.
- Repository root checked out.

## Environment variables

- `DATAFORGE_OWNER_URL` preferred; fallback to `DATABASE_URL`.
- Example: `export DATAFORGE_OWNER_URL="postgresql://dataforge_owner@localhost:5432/dataforge"`

## Applying the schema

```bash
chmod +x zfss/scripts/apply_schema.sh
DATAFORGE_OWNER_URL="postgresql://dataforge_owner@localhost:5432/dataforge" ./zfss/scripts/apply_schema.sh
```

The script applies the migrations in order and writes a log in `zfss/receipts/schema_apply_<timestamp>.log`.

## Verification

- List tables: `psql "$DATAFORGE_OWNER_URL" -c "\dt"`
- Confirm append-only trigger: `psql "$DATAFORGE_OWNER_URL" -c "UPDATE signals SET status='linked' WHERE false;"` should fail with the trigger error.
- For any inserted rows, verify history tables exist (`signal_status_history`, `signal_links`).
 - Run `zfss/scripts/db_status.sh` (after setting `DATABASE_URL`) for a quick PASS/FAIL report of the schema status and trigger presence.

The receipt log contains hostname, git SHA, filenames, and PASS/FAIL so auditors can confirm the schema run.
