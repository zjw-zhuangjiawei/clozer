# Development: Registry

**Summary**: In-memory data access layer with BTreeMap storage, secondary indexes, and dirty tracking.

**Why**: Provides fast CRUD operations with deterministic iteration order and efficient persistence.

---

## Registry Structure

Use `BTreeMap` and `BTreeSet` for deterministic ordering:

```rust
// From src/registry/word.rs
use crate::models::Word;
use crate::persistence::DbError;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct WordRegistry {
    pub(crate) words: BTreeMap<Uuid, Word>,
    pub(crate) dirty_ids: BTreeSet<Uuid>,
}
```

---

## CRUD Methods

Use simple, concise names:

| Method | Receiver | Returns | Description |
|--------|----------|---------|-------------|
| `add` | `&mut self` | - | Add a new entity |
| `get` | `&self` | `Option<&T>` | Get entity by ID |
| `get_mut` | `&mut self` | `Option<&mut T>` | Get mutable entity by ID |
| `delete` | `&mut self` | `bool` | Remove entity by ID |

```rust
impl WordRegistry {
    pub fn add(&mut self, word: Word) {
        self.words.insert(word.id, word.clone());
        self.dirty_ids.insert(word.id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Word> {
        self.words.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Word> {
        self.words.get_mut(&id)
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        if self.words.remove(&id).is_some() {
            self.dirty_ids.insert(id);
            true
        } else {
            false
        }
    }
}
```

---

## Iterator Methods

Follow Rust conventions:

| Method | Receiver | Yields |
|--------|----------|--------|
| `iter()` | `&self` | `&T` |
| `iter_mut()` | `&mut self` | `&mut T` |

```rust
impl WordRegistry {
    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Word)> {
        self.words.iter()
    }

    pub fn count(&self) -> usize {
        self.words.len()
    }

    pub fn exists(&self, id: Uuid) -> bool {
        self.words.contains_key(&id)
    }

    // Meaning ID management (syncs with MeaningRegistry)
    pub fn add_meaning(&mut self, word_id: Uuid, meaning_id: Uuid) -> bool {
        if let Some(word) = self.words.get_mut(&word_id) {
            word.meaning_ids.insert(meaning_id);
            self.dirty_ids.insert(word_id);
            true
        } else {
            false
        }
    }

    pub fn remove_meaning(&mut self, word_id: Uuid, meaning_id: Uuid) -> bool {
        if let Some(word) = self.words.get_mut(&word_id) {
            let removed = word.meaning_ids.remove(&meaning_id);
            if removed {
                self.dirty_ids.insert(word_id);
            }
            removed
        } else {
            false
        }
    }
}
```

---

## Dirty Tracking

Track modified entities for efficient persistence:

```rust
impl WordRegistry {
    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        !self.dirty_ids.is_empty()
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        let dirty_count = self.dirty_ids.len();
        if dirty_count == 0 {
            return Ok(());
        }

        tracing::info!("Flushing {} dirty words", dirty_count);

        let mut errors = 0;
        let dirty_ids: Vec<_> = self.dirty_ids.iter().copied().collect();
        for id in dirty_ids {
            if let Some(word) = self.words.get(&id) {
                let dto = crate::persistence::WordDto::from(word);
                match db.save_word(id, &dto) {
                    Ok(_) => {
                        tracing::debug!(word_id = %id, "Saved word");
                        self.dirty_ids.remove(&id);
                    }
                    Err(e) => {
                        errors += 1;
                        tracing::error!(word_id = %id, error = %e, "Failed to save word");
                    }
                }
            } else {
                match db.delete_word(id) {
                    Ok(_) => {
                        tracing::debug!(word_id = %id, "Deleted word");
                        self.dirty_ids.remove(&id);
                    }
                    Err(e) => {
                        errors += 1;
                        tracing::error!(word_id = %id, error = %e, "Failed to delete word");
                    }
                }
            }
        }
        if errors > 0 {
            tracing::warn!(errors = errors, "Some words failed to persist");
        } else {
            tracing::info!("Flushed {} words successfully", dirty_count);
        }
        Ok(())
    }
}
```

---

## Secondary Indexes

Maintain indexes for efficient lookups:

```rust
// MeaningRegistry has by_word and by_tag indexes
pub struct MeaningRegistry {
    meanings: BTreeMap<Uuid, Meaning>,
    dirty_ids: BTreeSet<Uuid>,
    by_word: BTreeMap<Uuid, BTreeSet<Uuid>>,   // word_id → meaning_ids
    by_tag: BTreeMap<Uuid, BTreeSet<Uuid>>,    // tag_id → meaning_ids
}

impl MeaningRegistry {
    pub fn iter_by_word(&self, word_id: Uuid) -> impl Iterator<Item = (&Uuid, &Meaning)> {
        self.by_word
            .get(&word_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.meanings.get(id).map(|m| (id, m)))
            })
            .unwrap_or_else(std::iter::empty)
    }

    pub fn iter_by_tag(&self, tag_id: Uuid) -> impl Iterator<Item = (&Uuid, &Meaning)> {
        self.by_tag
            .get(&tag_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.meanings.get(id).map(|m| (id, m)))
            })
            .unwrap_or_else(std::iter::empty)
    }
}
```

---

## Related Rules

- [Dev: Models](./dev-models.md) - Data structures
- [Dev: Persistence](./dev-persistence.md) - Database operations
- [Architecture Layers](./arch-layers.md) - Layer overview
