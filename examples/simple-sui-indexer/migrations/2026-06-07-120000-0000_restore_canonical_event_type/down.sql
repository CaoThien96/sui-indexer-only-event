-- Re-normalize event types to lowercase (previous storage format).
UPDATE package_events
SET event_type = lower(event_type);
