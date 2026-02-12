//! Queue DTO for serialization.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Queue item status encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum QueueItemStatusDto {
    Pending = 0,
    Processing = 1,
    Completed = 2,
    Failed = 3,
}

/// Queue item entity data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueItemDto {
    pub meaning_id: Uuid,
    #[serde(with = "QueueItemStatusDto")]
    pub status: QueueItemStatusDto,
}
