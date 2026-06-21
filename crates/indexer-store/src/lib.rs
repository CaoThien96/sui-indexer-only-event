pub mod composite;
pub mod kafka;
#[cfg(test)]
mod kafka_tests;
pub mod model;
pub mod postgres;
#[cfg(test)]
mod postgres_tests;
pub mod schema;

pub use composite::{CompositeConnection, CompositeStore};
pub use kafka::{
    FactTopic, KafkaFactReader, KafkaFactWriter, MessageEnvelope, compute_message_id, now_ms,
    parse_envelope,
};
pub use postgres::{DbArgs, PostgresStore};
