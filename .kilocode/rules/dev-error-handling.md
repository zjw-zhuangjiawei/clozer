# Development: Error Handling

**Summary**: Error handling patterns using thiserror and Result types.

**Why**: Provides consistent error handling across all layers with proper error propagation and context.

---

## Error Handling with thiserror

Use `thiserror` for custom error types with structured error messages.

### Define Errors

```rust
// From src/persistence/db/core.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(#[from] redb::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] rmp_serde::encode::Error),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] rmp_serde::decode::Error),

    #[error("Table not found: {0}")]
    TableNotFound(String),
}
```

### Usage in Functions

```rust
// Return Result for fallible operations
pub fn save_word(&self, id: Uuid, dto: &WordDto) -> Result<(), DbError> {
    let key = id.to_string();
    let value = rmp_serde::to_vec(dto)?;
    self.db.write(|tx| {
        let table = tx.open_table(WORDS_TABLE)?;
        table.insert(&key, &value)?;
        Ok(())
    })?;
    tracing::debug!("Saved word {}", id);
    Ok(())
}

// Propagate errors with ?
pub fn load_all(&mut self, db: &Db) -> Result<(), DbError> {
    for dto in db.iter_words()? {
        let word = Word::from(dto);
        self.words.insert(word.id, word);
    }
    Ok(())
}
```

---

## Error Handling Pattern

Log errors at appropriate levels with context, then handle or propagate.

```rust
// Registry persistence errors
let mut errors = 0;
for (id, word) in self.words.iter() {
    if let Err(e) = db.save_word(*id, word) {
        tracing::error!(word_id = %id, error = %e, "Failed to save word");
        errors += 1;
    }
}
if errors > 0 {
    tracing::warn!(errors = errors, "Some words failed to persist");
}
```

---

## Result Type Aliases

Use type aliases for common Result types to reduce repetition.

```rust
// In module
pub type Result<T> = std::result::Result<T, DbError>;

// Usage
pub fn save_word(&self, id: Uuid, dto: &WordDto) -> Result<()> { ... }
```

---

## Propagating Context with map_err

Add context when propagating errors up the stack.

```rust
pub fn load_config(&self) -> Result<AppConfig> {
    self.read_file()
        .map_err(|e| Error::Config(format!("Failed to read config: {}", e)))?
        .parse()
        .map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))
}
```

---

## Ignoring Errors with if let

For non-critical operations where errors can be safely ignored:

```rust
// Non-critical - just log and continue
if let Err(e) = self.save_dirty_entities() {
    tracing::warn!(error = %e, "Failed to persist some entities");
}
```

---

## Related Rules

- [Architecture Layers](./arch-layers.md) - Layer responsibilities
- [Dev Logging](./dev-logging.md) - Tracing patterns
- [Dev Models](./dev-models.md) - Data structures
