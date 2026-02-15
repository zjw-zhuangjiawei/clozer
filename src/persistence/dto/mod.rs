//! DTO module for serialization.
//!
//! Re-exports all DTO types.

pub use self::{
    cloze::{ClozeDto, ClozeSegmentDto},
    meaning::{MeaningDto, PartOfSpeechDto},
    tag::TagDto,
    word::WordDto,
};

pub mod cloze;
pub mod meaning;
pub mod tag;
pub mod word;
