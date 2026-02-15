//! Core database components: table definitions and serialization helpers.

use redb::TableDefinition;

/// Table definitions for redb 3.x.
pub const WORDS_TABLE: TableDefinition<[u8; 16], Vec<u8>> = TableDefinition::new("words");
pub const MEANINGS_TABLE: TableDefinition<[u8; 16], Vec<u8>> = TableDefinition::new("meanings");
pub const CLOZES_TABLE: TableDefinition<[u8; 16], Vec<u8>> = TableDefinition::new("clozes");
pub const TAGS_TABLE: TableDefinition<[u8; 16], Vec<u8>> = TableDefinition::new("tags");
// pub const PROVIDERS_TABLE: TableDefinition<[u8; 16], Vec<u8>> = TableDefinition::new("providers");
// pub const MODELS_TABLE: TableDefinition<[u8; 16], Vec<u8>> = TableDefinition::new("models");
// pub const QUEUE_TABLE: TableDefinition<[u8; 16], Vec<u8>> = TableDefinition::new("queue");

/// Serializes data to bytes using rmp-serde.
pub fn serialize<T: serde::Serialize>(data: &T) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    let bytes = rmp_serde::encode::to_vec(data)?;
    tracing::trace!(byte_count = bytes.len(), "Serialized data");
    Ok(bytes)
}

/// Deserializes data from bytes using rmp-serde.
pub fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, rmp_serde::decode::Error> {
    tracing::trace!(byte_count = bytes.len(), "Deserializing data");
    let result = rmp_serde::decode::from_slice(bytes);
    if result.is_err() {
        tracing::error!(error = ?result.as_ref().err(), "Deserialization failed");
    }
    result
}

/// Converts Uuid to bytes key for redb.
pub fn uuid_to_key(id: uuid::Uuid) -> [u8; 16] {
    id.into_bytes()
}

/// Converts bytes key back to Uuid.
pub fn key_to_uuid(key: [u8; 16]) -> uuid::Uuid {
    uuid::Uuid::from_bytes(key)
}
