# Error Handling

**Summary**: Error handling patterns using `thiserror` and `Result` types.

**Why**: Consistent error handling across layers with proper propagation and context.

---

## Define Errors with thiserror

```rust
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

---

## Use Result Type Aliases

```rust
pub type Result<T> = std::result::Result<T, DbError>;

pub fn save_word(&self, id: Uuid, dto: &WordDto) -> Result<()> { ... }
```

---

## Propagate Errors with `?`

```rust
pub fn load_all(&mut self, db: &Db) -> Result<()> {
    for dto in db.iter_words()? {
        let word = Word::from(dto);
        self.words.insert(word.id, word);
    }
    Ok(())
}
```

---

## Add Context with `map_err`

```rust
pub fn load_config(&self) -> Result<AppConfig> {
    self.read_file()
        .map_err(|e| Error::Config(format!("Failed to read config: {}", e)))?
        .parse()
        .map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))
}
```

---

## Ignore Non-Critical Errors with `if let`

```rust
if let Err(e) = self.save_dirty_entities() {
    tracing::warn!(error = %e, "Failed to persist some entities");
}
```
