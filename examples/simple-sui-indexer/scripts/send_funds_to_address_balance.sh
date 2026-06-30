#!/usr/bin/env bash
set -euo pipefail

# Manual helper: move SUI from a coin object into address balance.
# Usage:
#   scripts/send_funds_to_address_balance.sh <recipient_address> <amount_mist> [coin_object_id]
#
# Example:
#   scripts/send_funds_to_address_balance.sh 0xabc... 1000000000
#   scripts/send_funds_to_address_balance.sh 0xabc... 500000000 0xcoin...

RECIPIENT="${1:-}"
AMOUNT_MIST="${2:-}"
SOURCE_COIN="${3:-}"

if [[ -z "${RECIPIENT}" || -z "${AMOUNT_MIST}" ]]; then
  echo "Usage: $0 <recipient_address> <amount_mist> [coin_object_id]"
  exit 1
fi

if [[ -n "${SOURCE_COIN}" ]]; then
  echo "Sending from coin ${SOURCE_COIN} to address balance of ${RECIPIENT} (amount=${AMOUNT_MIST})"
  sui client ptb \
    --split-coins "@${SOURCE_COIN}" "[${AMOUNT_MIST}]" \
    --assign coin \
    --move-call 0x2::coin::send_funds '<0x2::sui::SUI>' coin "@${RECIPIENT}"
else
  echo "Sending from gas coin to address balance of ${RECIPIENT} (amount=${AMOUNT_MIST})"
  sui client ptb \
    --split-coins gas "[${AMOUNT_MIST}]" \
    --assign coin \
    --move-call 0x2::coin::send_funds '<0x2::sui::SUI>' coin "@${RECIPIENT}"
fi
