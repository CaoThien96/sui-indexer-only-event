#!/usr/bin/env bash
set -euo pipefail

BOOTSTRAP="${KAFKA_BOOTSTRAP:-localhost:9092}"
PARTITIONS="${KAFKA_PARTITIONS:-6}"
RETENTION_MS=$((7 * 24 * 60 * 60 * 1000))

TOPICS=(
  "dex.swap.raw.v1"
  "dex.pool.raw.v1"
  "token.metadata.raw.v1"
)

for topic in "${TOPICS[@]}"; do
  docker compose -f "$(dirname "$0")/../docker-compose.yml" exec -T kafka \
    /opt/kafka/bin/kafka-topics.sh \
    --bootstrap-server "${BOOTSTRAP}" \
    --create \
    --if-not-exists \
    --topic "${topic}" \
    --partitions "${PARTITIONS}" \
    --replication-factor 1 \
    --config retention.ms="${RETENTION_MS}"
  echo "Topic ready: ${topic}"
done

docker compose -f "$(dirname "$0")/../docker-compose.yml" exec -T kafka \
  /opt/kafka/bin/kafka-topics.sh --bootstrap-server "${BOOTSTRAP}" --list
