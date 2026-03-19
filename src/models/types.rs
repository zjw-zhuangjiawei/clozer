//! Newtype ID types for enhanced type safety.
//!
//! These types wrap UUIDs to prevent ID confusion between different entity types.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Newtype for Word entity IDs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct WordId(pub Uuid);

impl WordId {
    /// Create a new random WordId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for WordId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for WordId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<WordId> for Uuid {
    fn from(id: WordId) -> Self {
        id.0
    }
}

/// Newtype for Meaning entity IDs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct MeaningId(pub Uuid);

impl MeaningId {
    /// Create a new random MeaningId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for MeaningId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for MeaningId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<MeaningId> for Uuid {
    fn from(id: MeaningId) -> Self {
        id.0
    }
}

/// Newtype for Tag entity IDs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct TagId(pub Uuid);

impl TagId {
    /// Create a new random TagId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for TagId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TagId> for Uuid {
    fn from(id: TagId) -> Self {
        id.0
    }
}

/// Newtype for Cloze entity IDs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ClozeId(pub Uuid);

impl ClozeId {
    /// Create a new random ClozeId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ClozeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ClozeId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ClozeId> for Uuid {
    fn from(id: ClozeId) -> Self {
        id.0
    }
}

/// Newtype for Provider entity IDs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ProviderId(pub Uuid);

impl ProviderId {
    /// Create a new random ProviderId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ProviderId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ProviderId> for Uuid {
    fn from(id: ProviderId) -> Self {
        id.0
    }
}

/// Newtype for Model entity IDs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct ModelId(pub Uuid);

impl ModelId {
    /// Create a new random ModelId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ModelId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ModelId> for Uuid {
    fn from(id: ModelId) -> Self {
        id.0
    }
}
