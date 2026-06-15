#!/usr/bin/env python3
"""Analyze bot sandwich patterns per checkpoint from package_events.

Detects ordered triplets in the same checkpoint (consensus order):
  bot buy (sender=bot, atob=false)
  -> victim buy (sender!=bot, atob=false, same pool)
  -> bot sell (sender=bot, atob=true, same pool)

Usage:
  cd examples/simple-sui-indexer
  python3 scripts/analyze_bot_sandwich.py
  VERBOSE=1 python3 scripts/analyze_bot_sandwich.py
  LIMIT_CHECKPOINTS=10 python3 scripts/analyze_bot_sandwich.py
  REPORT_PATH=reports/bot_sandwich_report.md python3 scripts/analyze_bot_sandwich.py

Requires: ClickHouse with package_events, CLICKHOUSE_URL (or .env in cwd).
Output: markdown report file (default reports/bot_sandwich_report.md) + summary on stdout.
"""

from __future__ import annotations

import os
import sys
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "scripts"))
from ch_client import run_query_tsv  # noqa: E402


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
CLICKHOUSE_URL = os.environ.get("CLICKHOUSE_URL", "http://127.0.0.1:8123")
BOT_ADDRESS = os.environ.get(
    "BOT_ADDRESS",
    "0xf3981a28e88f86255713dada5d7b1ebb23b0b9e499e80fa1406bdd74c3364735",
).lower()
SWAP_EVENT_TYPE = os.environ.get(
    "SWAP_EVENT_TYPE",
    "0x1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb::pool::SwapEvent",
)
VERBOSE = os.environ.get("VERBOSE", "0") == "1"
LIMIT_CHECKPOINTS = int(os.environ.get("LIMIT_CHECKPOINTS", "0"))
REPORT_PATH = Path(
    os.environ.get("REPORT_PATH", "reports/bot_sandwich_report.md")
)


@dataclass
class Report:
    lines: list[str] = field(default_factory=list)

    def writeln(self, line: str = "") -> None:
        self.lines.append(line)

    def write(self, text: str) -> None:
        self.lines.extend(text.splitlines())

    def save(self, path: Path) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("\n".join(self.lines) + "\n", encoding="utf-8")


@dataclass(frozen=True)
class SwapEvent:
    tx_digest: str
    event_seq: int
    checkpoint: int
    tx_idx: int
    event_idx: int
    sender: str
    pool: str
    atob: bool
    amount_in: str

    @property
    def pos(self) -> tuple[int, int]:
        return (self.tx_idx, self.event_idx)


def run_ch(sql: str) -> str:
    return "\n".join(run_query_tsv(sql))


def sql_escape(value: str) -> str:
    return value.replace("'", "''").replace("\\", "\\\\")


def normalize_pool(pool: str) -> str:
    p = pool.lower()
    return p if p.startswith("0x") else f"0x{p}"


def parse_bool(raw: str) -> bool:
    return raw.strip().lower() in ("true", "t", "1")


def parse_swap_row(line: str) -> SwapEvent | None:
    parts = line.split("\t")
    if len(parts) < 9:
        return None
    (
        tx_digest,
        event_seq,
        checkpoint,
        tx_idx,
        event_idx,
        sender,
        pool,
        atob,
        amount_in,
    ) = parts[:9]
    if not pool:
        return None
    return SwapEvent(
        tx_digest=tx_digest,
        event_seq=int(event_seq),
        checkpoint=int(checkpoint),
        tx_idx=int(tx_idx),
        event_idx=int(event_idx),
        sender=(sender or "").lower(),
        pool=normalize_pool(pool),
        atob=parse_bool(atob),
        amount_in=amount_in or "0",
    )


def fetch_bot_checkpoints() -> list[int]:
    bot = sql_escape(BOT_ADDRESS)
    swap_type = sql_escape(SWAP_EVENT_TYPE)
    sql = f"""
SELECT DISTINCT checkpoint_sequence_number
FROM package_events FINAL
WHERE event_type = '{swap_type}'
  AND lower(sender) = '{bot}'
ORDER BY checkpoint_sequence_number
"""
    checkpoints: list[int] = []
    for line in run_ch(sql).splitlines():
        line = line.strip()
        if line:
            checkpoints.append(int(line))
    if LIMIT_CHECKPOINTS > 0:
        checkpoints = checkpoints[:LIMIT_CHECKPOINTS]
    return checkpoints


def fetch_checkpoint_swaps(checkpoint: int) -> list[SwapEvent]:
    swap_type = sql_escape(SWAP_EVENT_TYPE)
    sql = f"""
SELECT event_id_tx_digest,
       event_id_seq,
       checkpoint_sequence_number,
       transaction_sequence_in_checkpoint,
       event_sequence_in_transaction,
       sender,
       JSONExtractString(parsed_json, 'pool') AS pool,
       JSONExtractString(parsed_json, 'atob') AS atob,
       JSONExtractString(parsed_json, 'amount_in') AS amount_in
FROM package_events FINAL
WHERE event_type = '{swap_type}'
  AND checkpoint_sequence_number = {checkpoint}
ORDER BY transaction_sequence_in_checkpoint,
         event_sequence_in_transaction
"""
    events: list[SwapEvent] = []
    for line in run_ch(sql).splitlines():
        ev = parse_swap_row(line)
        if ev is not None:
            events.append(ev)
    return events


def is_bot_buy(ev: SwapEvent) -> bool:
    return ev.sender == BOT_ADDRESS and not ev.atob


def is_victim_buy(ev: SwapEvent) -> bool:
    return ev.sender != BOT_ADDRESS and not ev.atob


def is_victim_sell(ev: SwapEvent) -> bool:
    return ev.sender != BOT_ADDRESS and ev.atob


@dataclass
class BotBuyVictimSell:
    bot_buy: SwapEvent
    victim_sell: SwapEvent

    @property
    def same_checkpoint(self) -> bool:
        return self.bot_buy.checkpoint == self.victim_sell.checkpoint

    @property
    def checkpoint_delta(self) -> int:
        return self.victim_sell.checkpoint - self.bot_buy.checkpoint


@dataclass
class Sandwich:
    checkpoint: int
    pool: str
    bot_buy: SwapEvent
    victim_buy: SwapEvent
    bot_sell: SwapEvent


@dataclass
class CheckpointStats:
    checkpoint: int
    bot_buys: int = 0
    bot_sells: int = 0
    partial_bot_victim: int = 0
    full_sandwiches: int = 0
    sandwiches: list[Sandwich] | None = None


def is_bot_sell(ev: SwapEvent) -> bool:
    return ev.sender == BOT_ADDRESS and ev.atob


def fetch_all_swaps_from_checkpoint(min_checkpoint: int, max_checkpoint: int | None = None) -> list[SwapEvent]:
    swap_type = sql_escape(SWAP_EVENT_TYPE)
    max_clause = ""
    if max_checkpoint is not None:
        max_clause = f" AND checkpoint_sequence_number <= {max_checkpoint}"
    sql = f"""
SELECT event_id_tx_digest,
       event_id_seq,
       checkpoint_sequence_number,
       transaction_sequence_in_checkpoint,
       event_sequence_in_transaction,
       sender,
       JSONExtractString(parsed_json, 'pool') AS pool,
       JSONExtractString(parsed_json, 'atob') AS atob,
       JSONExtractString(parsed_json, 'amount_in') AS amount_in
FROM package_events FINAL
WHERE event_type = '{swap_type}'
  AND checkpoint_sequence_number >= {min_checkpoint}{max_clause}
ORDER BY checkpoint_sequence_number,
         transaction_sequence_in_checkpoint,
         event_sequence_in_transaction
"""
    events: list[SwapEvent] = []
    for line in run_ch(sql).splitlines():
        ev = parse_swap_row(line)
        if ev is not None:
            events.append(ev)
    return events


def analyze_bot_buy_victim_sell(all_events: list[SwapEvent]) -> tuple[list[BotBuyVictimSell], int]:
    """For each bot buy, first victim sell (same pool) in same or later checkpoint."""
    matches: list[BotBuyVictimSell] = []
    bot_buys_without_victim_sell = 0

    for i, ev in enumerate(all_events):
        if not is_bot_buy(ev):
            continue
        pool = ev.pool
        found = None
        for j in range(i + 1, len(all_events)):
            cand = all_events[j]
            if cand.pool == pool and is_victim_sell(cand):
                found = BotBuyVictimSell(bot_buy=ev, victim_sell=cand)
                break
        if found:
            matches.append(found)
        else:
            bot_buys_without_victim_sell += 1

    return matches, bot_buys_without_victim_sell


def format_bot_buy_victim_sell(m: BotBuyVictimSell) -> list[str]:
    where = "same checkpoint" if m.same_checkpoint else f"+{m.checkpoint_delta} checkpoints"
    return [
        f"- **pool** `{m.bot_buy.pool}` ({where})",
        f"  - **bot_buy** cp={m.bot_buy.checkpoint} `{m.bot_buy.tx_digest}` "
        f"(tx_idx={m.bot_buy.tx_idx}, ev_idx={m.bot_buy.event_idx}, amount_in={m.bot_buy.amount_in})",
        f"  - **victim_sell** cp={m.victim_sell.checkpoint} `{m.victim_sell.tx_digest}` "
        f"(sender=`{m.victim_sell.sender}`, tx_idx={m.victim_sell.tx_idx}, "
        f"ev_idx={m.victim_sell.event_idx}, amount_in={m.victim_sell.amount_in})",
    ]


def analyze_checkpoint(checkpoint: int, events: list[SwapEvent]) -> CheckpointStats:
    stats = CheckpointStats(checkpoint=checkpoint, sandwiches=[])

    i = 0
    while i < len(events):
        ev = events[i]
        if not is_bot_buy(ev):
            i += 1
            continue

        stats.bot_buys += 1
        pool = ev.pool

        victim_idx = None
        for j in range(i + 1, len(events)):
            cand = events[j]
            if cand.pool == pool and is_victim_buy(cand):
                victim_idx = j
                break

        if victim_idx is None:
            i += 1
            continue

        stats.partial_bot_victim += 1
        victim = events[victim_idx]

        sell_idx = None
        for k in range(victim_idx + 1, len(events)):
            cand = events[k]
            if cand.pool == pool and is_bot_sell(cand):
                sell_idx = k
                break

        if sell_idx is None:
            i += 1
            continue

        bot_sell = events[sell_idx]
        stats.bot_sells += 1
        stats.full_sandwiches += 1
        stats.sandwiches.append(
            Sandwich(
                checkpoint=checkpoint,
                pool=pool,
                bot_buy=ev,
                victim_buy=victim,
                bot_sell=bot_sell,
            )
        )
        i = sell_idx + 1

    return stats


def short_digest(digest: str, n: int = 10) -> str:
    if len(digest) <= n * 2:
        return digest
    return f"{digest[:n]}...{digest[-n:]}"


def format_sandwich(s: Sandwich) -> list[str]:
    return [
        f"- **pool** `{s.pool}`",
        f"  - **bot_buy** `{s.bot_buy.tx_digest}` "
        f"(tx_idx={s.bot_buy.tx_idx}, ev_idx={s.bot_buy.event_idx}, amount_in={s.bot_buy.amount_in})",
        f"  - **victim** `{s.victim_buy.tx_digest}` "
        f"(sender=`{s.victim_buy.sender}`, tx_idx={s.victim_buy.tx_idx}, "
        f"ev_idx={s.victim_buy.event_idx}, amount_in={s.victim_buy.amount_in})",
        f"  - **bot_sell** `{s.bot_sell.tx_digest}` "
        f"(tx_idx={s.bot_sell.tx_idx}, ev_idx={s.bot_sell.event_idx}, amount_in={s.bot_sell.amount_in})",
    ]


def format_sandwich_console(s: Sandwich) -> list[str]:
    return [
        f"  pool={short_digest(s.pool, 8)}",
        (
            f"    bot_buy   tx={short_digest(s.bot_buy.tx_digest)} "
            f"tx_idx={s.bot_buy.tx_idx} ev_idx={s.bot_buy.event_idx} "
            f"amount_in={s.bot_buy.amount_in}"
        ),
        (
            f"    victim    tx={short_digest(s.victim_buy.tx_digest)} "
            f"sender={short_digest(s.victim_buy.sender, 6)} "
            f"tx_idx={s.victim_buy.tx_idx} ev_idx={s.victim_buy.event_idx} "
            f"amount_in={s.victim_buy.amount_in}"
        ),
        (
            f"    bot_sell  tx={short_digest(s.bot_sell.tx_digest)} "
            f"tx_idx={s.bot_sell.tx_idx} ev_idx={s.bot_sell.event_idx} "
            f"amount_in={s.bot_sell.amount_in}"
        ),
    ]


def print_sandwich(s: Sandwich) -> None:
    for line in format_sandwich_console(s):
        print(line)


def main() -> int:
    try:
        checkpoints = fetch_bot_checkpoints()
    except RuntimeError as err:
        print(f"error: {err}", file=sys.stderr)
        return 1

    total_bot_buys = 0
    total_partial = 0
    total_sandwiches = 0
    all_sandwiches: list[Sandwich] = []
    checkpoint_stats: list[CheckpointStats] = []

    for cp in checkpoints:
        events = fetch_checkpoint_swaps(cp)
        cp_stats = analyze_checkpoint(cp, events)
        checkpoint_stats.append(cp_stats)
        total_bot_buys += cp_stats.bot_buys
        total_partial += cp_stats.partial_bot_victim
        total_sandwiches += cp_stats.full_sandwiches
        if cp_stats.sandwiches:
            all_sandwiches.extend(cp_stats.sandwiches)

        if VERBOSE and cp_stats.full_sandwiches > 0:
            pools = sorted({s.pool for s in cp_stats.sandwiches or []})
            pool_short = ", ".join(short_digest(p, 8) for p in pools)
            print(
                f"\nCheckpoint {cp}: sandwich x{cp_stats.full_sandwiches} pools=[{pool_short}]"
            )
            for sandwich in cp_stats.sandwiches or []:
                print_sandwich(sandwich)

    partial_only = total_partial - total_sandwiches

    min_cp = min(checkpoints) if checkpoints else 0
    max_cp = max(checkpoints) if checkpoints else 0
    global_swaps = fetch_all_swaps_from_checkpoint(min_cp, max_cp) if checkpoints else []
    buy_victim_sell, no_victim_sell = analyze_bot_buy_victim_sell(global_swaps)
    same_cp = sum(1 for m in buy_victim_sell if m.same_checkpoint)
    later_cp = len(buy_victim_sell) - same_cp

    report = build_report(
        checkpoints=checkpoints,
        checkpoint_stats=checkpoint_stats,
        all_sandwiches=all_sandwiches,
        total_bot_buys=total_bot_buys,
        total_sandwiches=total_sandwiches,
        partial_only=partial_only,
        buy_victim_sell=buy_victim_sell,
        no_victim_sell_after_buy=no_victim_sell,
        same_cp_victim_sell=same_cp,
        later_cp_victim_sell=later_cp,
    )
    report.save(REPORT_PATH)

    print(f"Bot: {BOT_ADDRESS}")
    print(f"SwapEvent: {SWAP_EVENT_TYPE}")
    print(f"Checkpoints with bot swaps: {len(checkpoints)}")
    print(f"Full sandwiches (bot_buy->victim_buy->bot_sell): {total_sandwiches}")
    print(f"Partial (bot_buy->victim_buy, no bot_sell in cp): {partial_only}")
    print(f"Bot buys total: {total_bot_buys}")
    print(f"Bot buy -> victim sell (same or later cp): {len(buy_victim_sell)}")
    print(f"  same checkpoint: {same_cp} | later checkpoint: {later_cp}")
    print(f"Bot buys with no victim sell after: {no_victim_sell}")
    print(f"Report written: {REPORT_PATH.resolve()}")

    if not VERBOSE and all_sandwiches:
        print("\nSandwich checkpoints (see report for full details):")
        by_cp: dict[int, int] = {}
        for s in all_sandwiches:
            by_cp[s.checkpoint] = by_cp.get(s.checkpoint, 0) + 1
        for cp, count in sorted(by_cp.items()):
            print(f"  checkpoint {cp}: {count}")

    return 0


def build_report(
    checkpoints: list[int],
    checkpoint_stats: list[CheckpointStats],
    all_sandwiches: list[Sandwich],
    total_bot_buys: int,
    total_sandwiches: int,
    partial_only: int,
    buy_victim_sell: list[BotBuyVictimSell],
    no_victim_sell_after_buy: int,
    same_cp_victim_sell: int,
    later_cp_victim_sell: int,
) -> Report:
    r = Report()
    generated = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")

    r.writeln("# Bot Sandwich Checkpoint Report")
    r.writeln()
    r.writeln(f"- Generated: {generated}")
    r.writeln(f"- Bot: `{BOT_ADDRESS}`")
    r.writeln(f"- SwapEvent: `{SWAP_EVENT_TYPE}`")
    r.writeln(f"- Ordering: `(transaction_sequence_in_checkpoint, event_sequence_in_transaction)`")
    r.writeln()
    r.writeln("## Summary")
    r.writeln()
    r.writeln("| Metric | Count |")
    r.writeln("|--------|------:|")
    r.writeln(f"| Checkpoints with bot swaps | {len(checkpoints)} |")
    r.writeln(f"| Bot buys (atob=false) | {total_bot_buys} |")
    r.writeln(f"| Full sandwiches (buy→victim→sell) | {total_sandwiches} |")
    r.writeln(f"| Partial (buy→victim, no sell in cp) | {partial_only} |")
    if total_bot_buys > 0:
        rate = 100.0 * total_sandwiches / total_bot_buys
        r.writeln(f"| Sandwich rate (full / bot buys) | {rate:.1f}% |")
        vrate = 100.0 * len(buy_victim_sell) / total_bot_buys
        r.writeln(f"| Bot buy → victim sell (same/later cp) | {len(buy_victim_sell)} ({vrate:.1f}%) |")
        r.writeln(f"| — same checkpoint | {same_cp_victim_sell} |")
        r.writeln(f"| — later checkpoint | {later_cp_victim_sell} |")
        r.writeln(f"| Bot buys, no victim sell after | {no_victim_sell_after_buy} |")
    r.writeln()

    r.writeln("## Bot buy → victim sell")
    r.writeln()
    r.writeln(
        "After each **bot buy** (`atob=false`), first **victim sell** (`atob=true`, "
        "sender≠bot, same pool) in the same checkpoint or any later checkpoint."
    )
    r.writeln()
    if not buy_victim_sell:
        r.writeln("_No bot buy → victim sell pairs found._")
    else:
        for i, match in enumerate(buy_victim_sell, 1):
            r.writeln(f"### Pair {i}")
            r.writeln()
            r.writeln("\n".join(format_bot_buy_victim_sell(match)))
            r.writeln()

    r.writeln("## Full sandwiches")
    r.writeln()
    if not all_sandwiches:
        r.writeln("_No full sandwiches found._")
    else:
        by_cp: dict[int, list[Sandwich]] = {}
        for s in all_sandwiches:
            by_cp.setdefault(s.checkpoint, []).append(s)
        for cp in sorted(by_cp):
            items = by_cp[cp]
            r.writeln(f"### Checkpoint {cp} ({len(items)} sandwich)")
            r.writeln()
            for i, sandwich in enumerate(items, 1):
                r.writeln(f"#### Sandwich {i}")
                r.writeln()
                r.writeln("\n".join(format_sandwich(sandwich)))
                r.writeln()

    r.writeln("## Per-checkpoint stats")
    r.writeln()
    r.writeln("| Checkpoint | Bot buys | Partial | Full sandwiches |")
    r.writeln("|------------|----------:|--------:|----------------:|")
    for stats in checkpoint_stats:
        if stats.bot_buys == 0 and stats.full_sandwiches == 0:
            continue
        partial = stats.partial_bot_victim - stats.full_sandwiches
        r.writeln(
            f"| {stats.checkpoint} | {stats.bot_buys} | {partial} | {stats.full_sandwiches} |"
        )
    r.writeln()

    return r


if __name__ == "__main__":
    sys.exit(main())
