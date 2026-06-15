#!/usr/bin/env python3
"""DEPRECATED — package_events retention is ClickHouse TTL tier (hot/cold local).

PostgreSQL no longer stores events. Use ClickHouse disk monitoring instead.
See examples/command.md.
"""

from __future__ import annotations

import sys


def main() -> int:
    print(
        "DEPRECATED: prune-package-events.py is no longer used.\n"
        "Events live in ClickHouse; TTL MOVE moves parts >3 days to cold HDD volume.",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
