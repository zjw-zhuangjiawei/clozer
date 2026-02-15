//! Persistence layer using redb and rmp-serde serialization.
//!
//! This module provides data persistence for the Clozer application.

pub mod db;
pub mod dto;

pub use crate::models::ClozeSegment;
pub use db::{Db, DbError};
pub use dto::{
    ClozeDto, ClozeSegmentDto, MeaningDto, PartOfSpeechDto,
    TagDto, WordDto,
};
