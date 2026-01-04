# Offline Verification Runbook

Purpose: execute `zfss/scripts/verify_migration.py` within a restricted environment that cannot hit PyPI. This runbook documents preconditions, offline dependency options, exact commands, expected outputs/failure modes, archival steps, and explicit HUMAN-REQUIRED gates.

## 1. Prerequisites

- Local PostgreSQL instance populated via the migration replay, accessible via the append-only role (`postgresql://dataforge_append:<pwd>@localhost:5432/dataforge`).
- Render snapshot directory present and readable (default `zfss_render_snapshot/` with all canonical CSVs).
- `python3` available and runnable inside the workspace (`python3 --version` should succeed).
- Project repo checked out; use relative paths to the scripts/docs.

## 2. Offline dependency installation

### Preferred (offline wheel)

1. Download `psycopg2-binary` wheel on another machine (matching the interpreter version, e.g., `psycopg2_binary-2.9.10-cp312-cp312-linux_x86_64.whl`).
2. Place the wheel inside the repository (e.g., `vendor/psycopg2_binary.whl`).
3. Create a venv and install from that wheel:
   ```bash
   python3 -m venv .venv_verify
   source .venv_verify/bin/activate
   pip install vendor/psycopg2_binary.whl
   ```

### Alternative (system package)

1. Check if an offline distro package is available:
   ```bash
   apt-cache policy python3-psycopg2
   ```
2. If it is present locally, install it without network:
   ```bash
   sudo apt install python3-psycopg2
   ```
   (This command must be permitted by the ops team; if `apt` cannot run offline, fall back to the wheel strategy).

## 3. Verification commands

1. Activate the venv created above (`source .venv_verify/bin/activate`) or ensure the installed `psycopg2` is on the default `python3` path.
2. Run the script:
   ```bash
   python3 zfss/scripts/verify_migration.py \
     --snapshot-dir ./zfss_render_snapshot \
     --database-url postgresql://dataforge_append:<append-password>@localhost:5432/dataforge
   ```

## 4. Expected outputs / failure modes

- **PASS:** script prints matching row/timestamp summaries and ends with “Migration verification passed…”; exit code 0.
- **Dependency failure:** script exits immediately with “psycopg2 is required…” or the pip install stage fails; indicates wheel/system package not installed.
- **Connection failure:** script raises `OperationalError` referencing Postgres connection; confirm local Postgres is running and credentials match.
- **Snapshot mismatch:** script lists row/timestamp/FK mismatches; re-export/replay and rerun.

## 5. Archival requirements

- Save the verification log (terminal output) as `logs/verify_migration.log`.
- Record the snapshot directory checksum (e.g., `sha256sum zfss_render_snapshot/*.csv > logs/snapshot.sha256`).
- Store the wheel/package used (if not built-in) alongside the archive so auditors see the offline dependency.

## 6. HUMAN-REQUIRED STOP POINTS

| Task | Notes |
| --- | --- |
| Render credential rotation | Update Render’s `forge-db` credentials and remove `DATABASE_URL` in `render-dataforge-only.yaml`. This cannot be done in this workspace because there’s no Render dashboard access. |
| Secret updates | Any new append/signal/read passwords must be seeded into vaults separately; do not store them in this repo. |
| Local Postgres availability | If the local DB is down, restart it on the host before rerunning verification; this script assumes it is reachable. |

After the above steps, circulate the archived logs + snapshot artifacts to the audit channel before proceeding to cutover documentation (`zfss/docs/authority_cutover.md`).
