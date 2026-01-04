#!/usr/bin/env python3
"""Verify Render→local DataForgeDB migration continuity (Prompt 6)."""

import argparse
import csv
import datetime
import sys
from pathlib import Path

try:
    import psycopg2
    from psycopg2 import sql
except ImportError as exc:
    raise SystemExit(
        "psycopg2 is required to run this verification script. "
        "Install it via `pip install psycopg2-binary`."
    ) from exc

TABLE_SPECS = [
    {"name": "users", "timestamps": ["created_at"]},
    {"name": "issues", "timestamps": ["created_at"]},
    {"name": "issue_status_history", "timestamps": ["changed_at"]},
    {"name": "signals", "timestamps": ["created_at"]},
    {"name": "signal_status_history", "timestamps": ["changed_at"]},
    {"name": "attachments", "timestamps": ["created_at"]},
    {"name": "decisions", "timestamps": ["decided_at"]},
    {"name": "artifacts", "timestamps": ["created_at"]},
    {"name": "responses", "timestamps": ["drafted_at"]},
    {"name": "response_approval_history", "timestamps": ["changed_at"]},
    {"name": "audit_log", "timestamps": ["created_at"]},
]

FK_CHECKS = [
    {
        "description": "signals.linked_issue_id → issues.id",
        "query": """
            SELECT 1
            FROM signals s
            WHERE s.linked_issue_id IS NOT NULL
              AND NOT EXISTS (
                  SELECT 1 FROM issues i WHERE i.id = s.linked_issue_id
              )
            LIMIT 1
        """,
    },
    {
        "description": "attachments.signal_id → signals.id",
        "query": """
            SELECT 1
            FROM attachments a
            WHERE NOT EXISTS (
                SELECT 1 FROM signals s WHERE s.id = a.signal_id
            )
            LIMIT 1
        """,
    },
    {
        "description": "decisions.issue_id → issues.id",
        "query": """
            SELECT 1
            FROM decisions d
            WHERE NOT EXISTS (
                SELECT 1 FROM issues i WHERE i.id = d.issue_id
            )
            LIMIT 1
        """,
    },
    {
        "description": "decisions.supersedes_id → decisions.id",
        "query": """
            SELECT 1
            FROM decisions d
            WHERE d.supersedes_id IS NOT NULL
              AND NOT EXISTS (
                  SELECT 1 FROM decisions d2 WHERE d2.id = d.supersedes_id
              )
            LIMIT 1
        """,
    },
    {
        "description": "artifacts.issue_id → issues.id",
        "query": """
            SELECT 1
            FROM artifacts a
            WHERE NOT EXISTS (
                SELECT 1 FROM issues i WHERE i.id = a.issue_id
            )
            LIMIT 1
        """,
    },
    {
        "description": "responses.signal_id → signals.id",
        "query": """
            SELECT 1
            FROM responses r
            WHERE r.signal_id IS NOT NULL
              AND NOT EXISTS (
                SELECT 1 FROM signals s WHERE s.id = r.signal_id
              )
            LIMIT 1
        """,
    },
    {
        "description": "responses.issue_id → issues.id",
        "query": """
            SELECT 1
            FROM responses r
            WHERE r.issue_id IS NOT NULL
              AND NOT EXISTS (
                SELECT 1 FROM issues i WHERE i.id = r.issue_id
              )
            LIMIT 1
        """,
    },
    {
        "description": "issue_status_history.issue_id → issues.id",
        "query": """
            SELECT 1
            FROM issue_status_history h
            WHERE NOT EXISTS (
                SELECT 1 FROM issues i WHERE i.id = h.issue_id
            )
            LIMIT 1
        """,
    },
    {
        "description": "signal_status_history.signal_id → signals.id",
        "query": """
            SELECT 1
            FROM signal_status_history h
            WHERE NOT EXISTS (
                SELECT 1 FROM signals s WHERE s.id = h.signal_id
            )
            LIMIT 1
        """,
    },
    {
        "description": "response_approval_history.response_id → responses.id",
        "query": """
            SELECT 1
            FROM response_approval_history h
            WHERE NOT EXISTS (
                SELECT 1 FROM responses r WHERE r.id = h.response_id
            )
            LIMIT 1
        """,
    },
]


def parse_timestamp(value: str):
    if not value:
        return None
    value = value.strip()
    if not value:
        return None
    if value.endswith("Z"):
        value = value[:-1] + "+00:00"
    try:
        return datetime.datetime.fromisoformat(value)
    except ValueError:
        for fmt in ("%Y-%m-%d %H:%M:%S%z", "%Y-%m-%d %H:%M:%S"):
            try:
                return datetime.datetime.strptime(value, fmt)
            except ValueError:
                continue
    raise ValueError(f"Unable to parse timestamp: {value!r}")


def read_snapshot_stats(table_path: Path, timestamp_fields):
    stats = {field: {"min": None, "max": None} for field in timestamp_fields}
    duplicates = []
    seen = set()
    count = 0

    with table_path.open(newline="", encoding="utf-8") as fh:
        reader = csv.DictReader(fh)
        for row in reader:
            count += 1
            row_id = row.get("id")
            if row_id:
                if row_id in seen:
                    duplicates.append(row_id)
                else:
                    seen.add(row_id)
            for field in timestamp_fields:
                ts_value = row.get(field)
                if ts_value:
                    parsed = parse_timestamp(ts_value)
                    current = stats[field]["min"]
                    if current is None or parsed < current:
                        stats[field]["min"] = parsed
                    current = stats[field]["max"]
                    if current is None or parsed > current:
                        stats[field]["max"] = parsed

    return {
        "count": count,
        "duplicates": duplicates,
        "timestamp_stats": stats,
    }


def load_local_stats(connection, table: str, timestamp_fields):
    results = {}
    with connection.cursor() as cur:
        cur.execute(sql.SQL("SELECT COUNT(*) FROM {}").format(sql.Identifier(table)))
        count = cur.fetchone()[0]
        dup_query = sql.SQL(
            "SELECT COUNT(*) - COUNT(DISTINCT id) FROM {}"
        ).format(sql.Identifier(table))
        cur.execute(dup_query)
        duplicate_count = cur.fetchone()[0]

        timestamp_stats = {}
        for field in timestamp_fields:
            query = sql.SQL(
                "SELECT MIN({col}) AS min_ts, MAX({col}) AS max_ts, "
                "COUNT(*) FILTER (WHERE {col} IS NULL) AS nulls "
                "FROM {tbl}"
            ).format(
                col=sql.Identifier(field),
                tbl=sql.Identifier(table),
            )
            cur.execute(query)
            min_ts, max_ts, nulls = cur.fetchone()
            timestamp_stats[field] = {
                "min": min_ts,
                "max": max_ts,
                "nulls": nulls,
            }

    return {
        "count": count,
        "duplicate_count": duplicate_count,
        "timestamp_stats": timestamp_stats,
    }


def format_ts(value):
    if value is None:
        return "NULL"
    return value.isoformat()


def main():
    parser = argparse.ArgumentParser(
        description="Verify the Render snapshot vs the local DataForgeDB (rows, timestamps, FK integrity)"
    )
    parser.add_argument(
        "--snapshot-dir",
        required=True,
        type=Path,
        help="Directory produced by zfss/scripts/export_render_snapshot.sh",
    )
    parser.add_argument(
        "--database-url",
        required=True,
        help="Local Postgres connection (append role) pointing at the new authoritative DataForgeDB",
    )

    args = parser.parse_args()
    snapshot_dir = args.snapshot_dir.resolve()
    if not snapshot_dir.is_dir():
        raise SystemExit(f"{snapshot_dir} does not exist or is not a directory")

    with psycopg2.connect(args.database_url) as conn:
        conn.autocommit = True
        failures = []

        print("== Row + Timestamp Verification ==")
        for spec in TABLE_SPECS:
            table = spec["name"]
            ts_fields = spec["timestamps"]
            csv_path = snapshot_dir / f"{table}.csv"
            if not csv_path.exists():
                failures.append(f"Missing snapshot file for table: {table}")
                continue

            snapshot = read_snapshot_stats(csv_path, ts_fields)
            local = load_local_stats(conn, table, ts_fields)

            print(f"- {table}: snapshot={snapshot['count']} rows, local={local['count']} rows")
            if snapshot["count"] != local["count"]:
                failures.append(f"{table}: row count mismatch ({snapshot['count']} vs {local['count']})")

            if snapshot["duplicates"]:
                failures.append(
                    f"{table}: snapshot contains duplicate ids {snapshot['duplicates'][:5]}..."
                )

            if local["duplicate_count"] > 0:
                failures.append(f"{table}: local duplicates detected ({local['duplicate_count']})")

            for field in ts_fields:
                snap = snapshot["timestamp_stats"][field]
                loc = local["timestamp_stats"][field]
                print(
                    f"  ts[{field}]: snapshot min={format_ts(snap['min'])}, max={format_ts(snap['max'])} "
                    f"| local min={format_ts(loc['min'])}, max={format_ts(loc['max'])} (nulls={loc['nulls']})"
                )
                if snap["min"] != loc["min"]:
                    failures.append(f"{table}.{field}: minimum timestamp differs")
                if snap["max"] != loc["max"]:
                    failures.append(f"{table}.{field}: maximum timestamp differs")
                if loc["nulls"] and snap["count"] > 0:
                    failures.append(f"{table}.{field}: {loc['nulls']} null timestamps")

        print("\n== Foreign Key Integrity ==")
        for check in FK_CHECKS:
            with conn.cursor() as cur:
                cur.execute(check["query"])
                if cur.fetchone():
                    failures.append(f"Foreign key violation: {check['description']}")
                    print(f"  ✗ {check['description']}")
                else:
                    print(f"  ✓ {check['description']}")

        if failures:
            print("\n== Failures ==")
            for issue in failures:
                print(f"  - {issue}")
            raise SystemExit("Migration verification failed (see failures above)")

    print("\nMigration verification passed. Row counts, timestamps, and foreign keys are consistent.")


if __name__ == "__main__":
    main()
