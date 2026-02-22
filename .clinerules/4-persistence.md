# Persistence

**Summary**: Persistent storage layer using redb database with MessagePack serialization.

**Why**: Provides durable storage for entities with efficient serialization and bidirectional conversion between models and DTOs.

---

## Database Structure

The persistence layer uses redb for embedded database storage:

```rust
// From src/persistence/mod.rs
use redb::{Database, TableDefinition};

pub struct Db {
    db: Database,
}

// Table definitions
const WORDS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("words");
const MEANINGS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("meanings");
const CLOZES_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("clozes");
const TAGS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("tags");
```

---

## DTO Pattern

Use DTOs (Data Transfer Objects) for serialization:

```rust
// From src/persistence/dto/word.rs
use crate::models::Word;
use langtag::LangTagBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Word entity data (matches Word model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordDto {
    pub id: Uuid,
    pub content: String,
    pub meaning_ids: Vec<Uuid>,
    pub language: Option<LangTagBuf>,
}
```

### Bidirectional Conversion

```rust
// From model to DTO
impl From<&Word> for WordDto {
    fn from(word: &Word) -> Self {
        WordDto {
            id: word.id,
            content: word.content.clone(),
            meaning_ids: word.meaning_ids.iter().cloned().collect(),
            language: word.language.clone(),
        }
    }
}

// From DTO to model
impl From<WordDto> for Word {
    fn from(dto: WordDto) -> Self {
        Word {
            id: dto.id,
            content: dto.content,
            meaning_ids: dto.meaning_ids.into_iter().collect(),
            language: dto.language,
        }
    }
}
```

---

## Database Operations

### Save Entity

```rust
// From src/persistence/db/words.rs
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
```

### Load Entity

```rust
pub fn get_word(&self, id: Uuid) -> Result<Option<WordDto>, DbError> {
    let key = id.to_string();
    self.db.read(|tx| {
        let table = tx.open_table(WORDS_TABLE)?;
        if let Some(value) = table.get(&key)? {
            let dto: WordDto = rmp_serde::from_slice(value.value())?;
            Ok(Some(dto))
        } else {
            Ok(None)
        }
    })
}
```

### Delete Entity

```rust
pub fn delete_word(&self, id: Uuid) -> Result<(), DbError> {
    let key = id.to_string();
    self.db.write(|tx| {
        let table = tx.open_table(WORDS_TABLE)?;
        table.remove(&key)?;
        Ok(())
    })?;
    tracing::debug!("Deleted word {}", id);
    Ok(())
}
```

### Iterate All

```rust
pub fn iter_words(&self) -> Result<impl Iterator<Item = WordDto>, DbError> {
    let mut results = Vec::new();
    self.db.read(|tx| {
        let table = tx.open_table(WORDS_TABLE)?;
        for entry in table.iter()? {
            let (_, value) = entry?;
            let dto: WordDto = rmp_serde::from_slice(value)?;
            results.push(dto);
        }
        Ok(results)
    })?;
    Ok(results.into_iter())
}
```

---

## Serialization Notes

- Uses `rmp-serde` for MessagePack serialization
- `Vec<Uuid>` for serialization (converts to/from `BTreeSet<Uuid>` in model)
- UUIDs stored as strings in database keys

---

## Related Rules

- [Models](./2-models.md) - Data structures
- [Registry](./3-registry.md) - In-memory storage
- [Architecture](./1-architecture.md) - Layer overview
