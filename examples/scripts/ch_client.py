"""Minimal ClickHouse HTTP client for scripts."""

from __future__ import annotations

import base64
import os
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path


def _examples_root() -> Path:
    return Path(__file__).resolve().parent.parent


def _parse_env_file(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, _, value = line.partition("=")
        values[key.strip()] = value.strip()
    return values


def load_clickhouse_env() -> None:
    """Load CLICKHOUSE_* from repo .env files (does not override existing env)."""
    root = _examples_root()
    for rel in (
        "simple-sui-indexer/.env",
        "rpc-service/.env",
        "clickhouse/.env",
    ):
        path = root / rel
        if not path.is_file():
            continue
        for key, value in _parse_env_file(path).items():
            if key.startswith("CLICKHOUSE_"):
                os.environ.setdefault(key, value)


def clickhouse_url() -> str:
    load_clickhouse_env()
    return os.environ.get("CLICKHOUSE_URL", "http://127.0.0.1:8123").rstrip("/")


def clickhouse_database() -> str:
    load_clickhouse_env()
    return os.environ.get("CLICKHOUSE_DATABASE", "sui_indexer")


def _auth_header() -> dict[str, str]:
    load_clickhouse_env()
    user = os.environ.get("CLICKHOUSE_USER")
    if not user or "CLICKHOUSE_PASSWORD" not in os.environ:
        return {}
    password = os.environ["CLICKHOUSE_PASSWORD"]
    token = base64.b64encode(f"{user}:{password}".encode()).decode("ascii")
    return {"Authorization": f"Basic {token}"}


def run_query(sql: str, *, database: str | None = None) -> str:
    db = database or clickhouse_database()
    params = urllib.parse.urlencode({"database": db})
    url = f"{clickhouse_url()}/?{params}"
    headers = {
        "Content-Type": "text/plain; charset=utf-8",
        **_auth_header(),
    }
    req = urllib.request.Request(
        url,
        data=sql.encode("utf-8"),
        method="POST",
        headers=headers,
    )
    if "Authorization" not in headers:
        raise RuntimeError(
            "ClickHouse auth required: set CLICKHOUSE_USER and CLICKHOUSE_PASSWORD "
            f"(e.g. in {_examples_root() / 'simple-sui-indexer' / '.env'})"
        )
    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            return resp.read().decode("utf-8")
    except urllib.error.HTTPError as exc:
        body = exc.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"ClickHouse query failed ({exc.code}): {body.strip()}") from exc


def run_query_tsv(sql: str) -> list[str]:
    out = run_query(sql + " FORMAT TabSeparated")
    return [line for line in out.splitlines() if line.strip()]
