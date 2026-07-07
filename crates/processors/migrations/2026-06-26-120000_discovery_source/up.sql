ALTER TABLE pools ADD COLUMN IF NOT EXISTS discovery_source TEXT NOT NULL DEFAULT 'pool_create';
ALTER TABLE tokens ADD COLUMN IF NOT EXISTS metadata_source TEXT NOT NULL DEFAULT 'indexer_metadata';

UPDATE tokens
SET metadata_source = 'stub'
WHERE name IS NULL AND symbol IS NULL AND metadata_source = 'indexer_metadata';
