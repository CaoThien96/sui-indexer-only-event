CREATE UNIQUE INDEX IF NOT EXISTS bot_processed_events_tx_event_idx
    ON bot_processed_events (tx_digest, event_seq);

CREATE UNIQUE INDEX IF NOT EXISTS bot_processed_swaps_tx_event_idx
    ON bot_processed_swaps (tx_digest, event_seq);
