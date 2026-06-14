#!/usr/bin/env python3
"""Delete old rows from package_events, keeping the last N days (by timestamp_ms).

Uses batched DELETE to avoid long locks. Safe to run while indexer + rpc-service are up.

Usage:
  ./scripts/prune-package-events.sh
  RETENTION_DAYS=3 BATCH_SIZE=5000 ./scripts/prune-package-events.sh
  ./scripts/prune-package-events.sh --dry-run
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import time
from datetime import datetime, timedelta, timezone


DEFAULT_DATABASE_URL = "postgres://postgres:postgres@localhost:5432/sui_indexer"
DEFAULT_RETENTION_DAYS = 3
DEFAULT_BATCH_SIZE = 10_000


def load_dotenv(path: str) -> None:
    if not os.path.isfile(path):
        return
    with open(path, encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, _, value = line.partition("=")
            key = key.strip()
            value = value.strip().strip("'").strip('"')
            if key and key not in os.environ:
                os.environ[key] = value


def psql(database_url: str, sql: str) -> str:
    result = subprocess.run(
        ["psql", database_url, "-v", "ON_ERROR_STOP=1", "-At", "-c", sql],
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def main() -> int:
    parser = argparse.ArgumentParser(description="Prune package_events older than N days")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Only print how many rows would be deleted",
    )
    parser.add_argument(
        "--retention-days",
        type=int,
        default=int(os.environ.get("RETENTION_DAYS", DEFAULT_RETENTION_DAYS)),
        help=f"Keep events with timestamp_ms within this many days (default {DEFAULT_RETENTION_DAYS})",
    )
    parser.add_argument(
        "--batch-size",
        type=int,
        default=int(os.environ.get("BATCH_SIZE", DEFAULT_BATCH_SIZE)),
        help=f"Rows per DELETE batch (default {DEFAULT_BATCH_SIZE})",
    )
    args = parser.parse_args()

    if args.retention_days < 1:
        print("retention-days must be >= 1", file=sys.stderr)
        return 1
    if args.batch_size < 1:
        print("batch-size must be >= 1", file=sys.stderr)
        return 1

    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    load_dotenv(os.path.join(root, "simple-sui-indexer", ".env"))
    load_dotenv(os.path.join(root, ".env"))

    database_url = os.environ.get("DATABASE_URL", DEFAULT_DATABASE_URL)
    cutoff = datetime.now(timezone.utc) - timedelta(days=args.retention_days)
    cutoff_ms = int(cutoff.timestamp() * 1000)

    print(f"DATABASE_URL={database_url.split('@')[-1]}")  # hide credentials
    print(f"retention_days={args.retention_days} cutoff_utc={cutoff.isoformat()} cutoff_ms={cutoff_ms}")
    print(f"batch_size={args.batch_size}")

    count_sql = f"""
        SELECT COUNT(*)::bigint
        FROM package_events
        WHERE timestamp_ms IS NOT NULL
          AND timestamp_ms < {cutoff_ms};
    """
    to_delete = int(psql(database_url, count_sql) or "0")
    null_ts_sql = """
        SELECT COUNT(*)::bigint
        FROM package_events
        WHERE timestamp_ms IS NULL;
    """
    null_ts = int(psql(database_url, null_ts_sql) or "0")

    print(f"rows_to_delete={to_delete} rows_with_null_timestamp_ms={null_ts} (null rows are skipped)")

    if args.dry_run:
        print("dry-run: no rows deleted")
        return 0

    if to_delete == 0:
        print("nothing to prune")
        return 0

    deleted_total = 0
    batch_num = 0
    started = time.perf_counter()

    delete_batch_sql = f"""
        WITH doomed AS (
            SELECT event_id_tx_digest, event_id_seq
            FROM package_events
            WHERE timestamp_ms IS NOT NULL
              AND timestamp_ms < {cutoff_ms}
            LIMIT {args.batch_size}
        ),
        deleted AS (
            DELETE FROM package_events pe
            USING doomed d
            WHERE pe.event_id_tx_digest = d.event_id_tx_digest
              AND pe.event_id_seq = d.event_id_seq
            RETURNING 1
        )
        SELECT COUNT(*)::bigint FROM deleted;
    """

    while True:
        batch_deleted = int(psql(database_url, delete_batch_sql) or "0")
        if batch_deleted == 0:
            break
        deleted_total += batch_deleted
        batch_num += 1
        print(f"batch {batch_num}: deleted={batch_deleted} total_deleted={deleted_total}")

    elapsed = time.perf_counter() - started
    print(f"done deleted={deleted_total} elapsed_sec={elapsed:.1f}")

  # optional light analyze
    try:
        psql(database_url, "ANALYZE package_events;")
        print("ANALYZE package_events completed")
    except subprocess.CalledProcessError as error:
        print(f"ANALYZE failed (non-fatal): {error}", file=sys.stderr)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
