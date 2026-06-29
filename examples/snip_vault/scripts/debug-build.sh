#!/usr/bin/env bash
# #region agent log
LOG="/Users/thiencao/bot-snip/.cursor/debug-d27706.log"
TS=$(($(date +%s) * 1000))
OUT=$(cd "$(dirname "$0")/.." && sui move build --build-env mainnet 2>&1)
EC=$?
# Hypothesis A: cetusclmm vs cetus_clmm address name mismatch
A=$(echo "$OUT" | grep -c "address 'cetusclmm' is not assigned" || true)
# Hypothesis B: legacy Sui/MoveStdlib git deps (should fail on CLI v1.63+)
B=$(echo "$OUT" | grep -c "legacy system name\|Packages with old dependencies" || true)
# Hypothesis C: missing framework linkage without manual addresses
C=$(echo "$OUT" | grep -c "address 'sui' is not assigned\|address 'std' is not assigned" || true)
# Hypothesis D: other compile errors
D=$(echo "$OUT" | grep -c "error\[E" || true)
printf '{"sessionId":"d27706","runId":"%s","hypothesisId":"summary","location":"scripts/debug-build.sh","message":"build result (skill v1.63+ format)","data":{"exitCode":%s,"legacyDepErrors":%s,"frameworkAddrErrors":%s,"totalErrors":%s,"suiVersion":"%s"},"timestamp":%s}\n' \
  "${RUN_ID:-pre-fix}" "$EC" "$B" "$C" "$D" "$(sui --version 2>/dev/null || echo unknown)" "$TS" >> "$LOG"
exit $EC
# #endregion
