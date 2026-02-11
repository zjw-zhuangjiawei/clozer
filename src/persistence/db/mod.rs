//! Database operations using redb.
//!
//! Provides CRUD operations for all entity types.

use std::path::Path;

use redb::{Database, ReadableDatabase};

mod core;
pub use core::*;
mod clozes;
mod meanings;
mod models;
mod providers;
mod queue;
mod tags;
mod words;

/// Database errors.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Redb(#[from] redb::DatabaseError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] postcard::Error),

    #[error("Transaction error: {0}")]
    Transaction(#[from] redb::TransactionError),

    #[error("Table error: {0}")]
    Table(#[from] redb::TableError),

    #[error("Storage error: {0}")]
    Storage(#[from] redb::StorageError),
}

/// Main database wrapper.
pub struct Db {
    db: Database,
}

impl Db {
    /// Opens or creates the database at the given path.
    pub fn new(path: &Path) -> Result<Self, DbError> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = Database::create(path)?;

        Ok(Self { db })
    }

    /// Gets a read transaction.
    fn read(&self) -> Result<redb::ReadTransaction, redb::TransactionError> {
        self.db.begin_read()
    }

    /// Gets a write transaction.
    fn write(&self) -> Result<redb::WriteTransaction, redb::TransactionError> {
        self.db.begin_write()
    }
}
