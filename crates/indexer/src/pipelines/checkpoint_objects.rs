//! Checkpoint object helpers — pattern from Mysten Walrus custom indexer and
//! `sui-indexer-alt-consistent-store` (see docs.sui.io Walrus custom indexer guide).

use std::collections::{BTreeMap, HashSet, btree_map::Entry};

use anyhow::Context;
use sui_indexer_alt_framework::types::{
    base_types::{ObjectID, SequenceNumber},
    effects::TransactionEffectsAPI,
    full_checkpoint_content::Checkpoint,
    object::Object,
};

/// Objects that existed before this checkpoint began (first input version per object id).
pub fn checkpoint_input_objects(
    checkpoint: &Checkpoint,
) -> anyhow::Result<BTreeMap<ObjectID, &Object>> {
    let mut output_objects_seen = HashSet::new();
    let mut checkpoint_input_objects = BTreeMap::new();

    for tx in &checkpoint.transactions {
        let input_objects_map: BTreeMap<(ObjectID, SequenceNumber), &Object> = tx
            .input_objects(&checkpoint.object_set)
            .map(|obj| ((obj.id(), obj.version()), obj))
            .collect();

        for change in tx.effects.object_changes() {
            let id = change.id;
            let Some(version) = change.input_version else {
                continue;
            };

            if output_objects_seen.contains(&id) {
                continue;
            }

            let Entry::Vacant(entry) = checkpoint_input_objects.entry(id) else {
                continue;
            };

            let input_obj = input_objects_map
                .get(&(id, version))
                .copied()
                .with_context(|| {
                    format!("object {id} at version {version} missing from checkpoint object_set")
                })?;

            entry.insert(input_obj);
        }

        for change in tx.effects.object_changes() {
            if change.output_version.is_some() {
                output_objects_seen.insert(change.id);
            }
        }
    }

    Ok(checkpoint_input_objects)
}

/// Live output objects at the end of the checkpoint (keyed by object id).
pub fn checkpoint_output_objects(
    checkpoint: &Checkpoint,
) -> anyhow::Result<BTreeMap<ObjectID, &Object>> {
    let mut output_objects = BTreeMap::new();

    for tx in &checkpoint.transactions {
        let output_objects_map: BTreeMap<_, _> = tx
            .output_objects(&checkpoint.object_set)
            .map(|obj| ((obj.id(), obj.version()), obj))
            .collect();

        for change in tx.effects.object_changes() {
            let id = change.id;
            output_objects.remove(&id);

            let Some(version) = change.output_version else {
                continue;
            };

            let output_object = output_objects_map
                .get(&(id, version))
                .copied()
                .with_context(|| format!("object {id} at version {version} missing from object_set"))?;

            output_objects.insert(id, output_object);
        }
    }

    Ok(output_objects)
}
