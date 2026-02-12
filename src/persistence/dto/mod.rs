//! DTO module for serialization.
//!
//! Re-exports all DTO types.

pub use self::{
    cloze::{ClozeDto, ClozeSegmentDto},
    meaning::{MeaningDto, PartOfSpeechDto},
    model::ModelDto,
    provider::{ProviderDto, ProviderTypeDto},
    queue::{QueueItemDto, QueueItemStatusDto},
    tag::TagDto,
    word::WordDto,
};

pub mod cloze;
pub mod meaning;
pub mod model;
pub mod provider;
pub mod queue;
pub mod tag;
pub mod word;
