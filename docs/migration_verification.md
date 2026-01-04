# Migration Verification (Prompt 6)

Purpose: prove the Render→local replay kept the ledger continuous by checking row counts, timestamp bounds, and foreign key integrity for every canonical and support table.

## Verification script

- Script: `zfss/scripts/verify_migration.py`
- Dependencies: `psycopg2-binary` (install via `pip install psycopg2-binary` if not already available).
- Input: the rendered CSV snapshot (default directory from Prompt 4 is `zfss_render_snapshot/`) and the append-role Postgres connection (`ZFSS_DATABASE_URL`/`LOCAL_DATABASE_URL`).

Use it like:

```bash
python zfss/scripts/verify_migration.py \
  --snapshot-dir ./zfss_render_snapshot \
  --database-url postgresql://dataforge_append:<append-password>@localhost:5432/dataforge
```

The script performs:

1. **Row counts + timestamps** – compares each table’s row count and `MIN/MAX` timestamp (e.g., `created_at`, `decided_at`, `changed_at`) between the Render snapshot and the local database; discrepancies cause failure.
2. **Duplicate detection** – ensures neither the snapshot nor the local copy contains duplicate `id` values.
3. **Foreign key integrity** – queries known relationships (issues → signals, decisions → issues, responses → signals/issues, history tables → canonical parents) and fails if any reference is missing.

Failures are reported with one-line reasons; the script exits non-zero when any mismatch is detected so the migration cannot be claimed as complete until resolved.

## Next steps after verification

- Archive both the snapshot and verification log for auditors (e.g., `tar -czf migration_verify.tar.gz zfss_render_snapshot/ verify.log`).
- If the script fails, review the reported tables, re-export the relevant CSV, re-import, and rerun this verification before advancing to Prompt 7 (authority cutover).
