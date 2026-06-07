-- Restore canonical Move event type strings (fullnode casing) from indexed event JSON.
UPDATE package_events
SET event_type = json->>'type'
WHERE json->>'type' IS NOT NULL
  AND json->>'type' <> ''
  AND event_type IS DISTINCT FROM json->>'type';
