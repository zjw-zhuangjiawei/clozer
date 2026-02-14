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

/// Serializes data to bytes using postcard.
pub fn serialize<T: serde::Serialize>(data: &T) -> Result<Vec<u8>, postcard::Error> {
    postcard::to_stdvec(data)
}

/// Deserializes data from bytes using postcard.
pub fn deserialize<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, postcard::Error> {
    postcard::from_bytes(bytes)
}

/// Converts Uuid to bytes key for redb.
pub fn uuid_to_key(id: uuid::Uuid) -> [u8; 16] {
    *id.as_bytes()
}

/// Converts bytes key back to Uuid.
pub fn key_to_uuid(key: [u8; 16]) -> uuid::Uuid {
    uuid::Uuid::from_bytes_le(key)
}
