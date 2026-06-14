#!/usr/bin/env python3
"""Post-hoc trap outcome stats for a user-provided pool_id.

Counts competitor bot buys/sells on the trap pool from package_events.
Not used for live trap triggering.

Usage:
  cd examples/sandwich-trap
  python3 scripts/analyze_trap_outcome.py

Env:
  DATABASE_URL, POOL_ID, BOT_ADDRESS, SWAP_EVENT_TYPE, REPORT_PATH
"""

from __future__ import annotations

import os
import subprocess
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path


def load_dotenv() -> None:
    env_path = Path.cwd() / ".env"
    if not env_path.is_file():
        return
    for line in env_path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, _, value = line.partition("=")
        os.environ.setdefault(key.strip(), value.strip())


load_dotenv()

DATABASE_URL = os.environ.get(
    "DATABASE_URL", "postgres://postgres:postgres@localhost:5432/sui_indexer"
)
BOT_ADDRESS = os.environ.get(
    "BOT_ADDRESS",
    "0xf3981a28e88f86255713dada5d7b1ebb23b0b9e499e80fa1406bdd74c3364735",
).lower()
POOL_ID = os.environ.get("POOL_ID", "")
SWAP_EVENT_TYPE = os.environ.get(
    "SWAP_EVENT_TYPE",
    "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent",
)
REPORT_PATH = Path(os.environ.get("REPORT_PATH", "reports/trap_outcome_report.md"))


@dataclass
class Report:
    lines: list[str] = field(default_factory=list)

    def writeln(self, line: str = "") -> None:
        self.lines.append(line)

    def save(self, path: Path) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("\n".join(self.lines) + "\n", encoding="utf-8")


def sql_escape(value: str) -> str:
    return value.replace("'", "''")


def run_psql(query: str) -> str:
    result = subprocess.run(
        ["psql", DATABASE_URL, "-t", "-A", "-F", "\t", "-c", query],
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        print(result.stderr, file=sys.stderr)
        sys.exit(1)
    return result.stdout.strip()


def main() -> None:
    if not POOL_ID:
        print("POOL_ID env var is required", file=sys.stderr)
        sys.exit(1)

    pool = sql_escape(POOL_ID.lower())
    bot = sql_escape(BOT_ADDRESS)
    event_type = sql_escape(SWAP_EVENT_TYPE)

    counts_query = f"""
SELECT
  COUNT(*) FILTER (WHERE lower(sender) = '{bot}' AND parsed_json->>'atob' = 'false') AS bot_buys,
  COUNT(*) FILTER (WHERE lower(sender) = '{bot}' AND parsed_json->>'atob' = 'true') AS bot_sells,
  COUNT(*) FILTER (WHERE lower(sender) != '{bot}' AND parsed_json->>'atob' = 'false') AS other_buys,
  COUNT(*) FILTER (WHERE lower(sender) != '{bot}' AND parsed_json->>'atob' = 'true') AS other_sells
FROM package_events
WHERE event_type = '{event_type}'
  AND lower(parsed_json->>'pool') = '{pool}';
"""
    row = run_psql(counts_query)
    parts = row.split("\t") if row else ["0", "0", "0", "0"]
    while len(parts) < 4:
        parts.append("0")
    bot_buys, bot_sells, other_buys, other_sells = parts[:4]

    stuck = max(0, int(bot_buys) - int(bot_sells))

    report = Report()
    report.writeln("# Trap Outcome Report")
    report.writeln()
    report.writeln(f"- Generated: {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M:%S UTC')}")
    report.writeln(f"- Pool: `{POOL_ID}`")
    report.writeln(f"- Bot: `{BOT_ADDRESS}`")
    report.writeln(f"- SwapEvent: `{SWAP_EVENT_TYPE}`")
    report.writeln()
    report.writeln("## Summary")
    report.writeln()
    report.writeln("| Metric | Count |")
    report.writeln("|--------|------:|")
    report.writeln(f"| Bot buys (atob=false) | {bot_buys} |")
    report.writeln(f"| Bot sells (atob=true) | {bot_sells} |")
    report.writeln(f"| Bot buys − sells (stuck proxy) | {stuck} |")
    report.writeln(f"| Other buys | {other_buys} |")
    report.writeln(f"| Other sells | {other_sells} |")

    report.save(REPORT_PATH)
    print(f"Report written to {REPORT_PATH}")
    print(f"bot_buys={bot_buys} bot_sells={bot_sells} stuck_proxy={stuck}")


if __name__ == "__main__":
    main()
