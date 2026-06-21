#!/usr/bin/env bash
set -euo pipefail

BOOTSTRAP="${KAFKA_BOOTSTRAP:-localhost:9092}"
PARTITIONS="${KAFKA_PARTITIONS:-6}"
RETENTION_MS=$((7 * 24 * 60 * 60 * 1000))

TOPICS=(
  "dex.swap.raw.v1"
  "dex.pool.raw.v1"
  "token.metadata.raw.v1"
  "dex.swap.normalized.v1"
)

COMPOSE_FILE="$(dirname "$0")/../docker-compose.yml"
ENV_FILE="$(dirname "$0")/../../.env"
COMPOSE_ARGS=(-f "$COMPOSE_FILE")
if [[ -f "$ENV_FILE" ]]; then
  COMPOSE_ARGS+=(--env-file "$ENV_FILE")
fi

for topic in "${TOPICS[@]}"; do
  docker compose "${COMPOSE_ARGS[@]}" exec -T kafka \
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

docker compose "${COMPOSE_ARGS[@]}" exec -T kafka \
  /opt/kafka/bin/kafka-topics.sh --bootstrap-server "${BOOTSTRAP}" --list
