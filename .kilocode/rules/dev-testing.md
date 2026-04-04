# Development: Testing

**Summary**: Testing patterns and conventions for the Clozer project.

**Why**: Ensures code quality through consistent testing approaches across unit, integration, and UI layers.

---

## Test Organization

Tests are co-located with the code they test using the `#[cfg(test)]` module pattern:

```
src/
├── models/
│   ├── word.rs
│   └── word_test.rs  # Unit tests for word module
├── registry/
│   └── mod.rs
└── ...
```

---

## Unit Tests with #[cfg(test)]

```rust
// From src/models/cloze.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_sentence() {
        let segments = Cloze::parse_from_sentence("The [cat] sat on the [mat]");
        assert_eq!(segments.len(), 5);
        assert_eq!(segments[0], ClozeSegment::Text("The ".to_string()));
        assert_eq!(segments[1], ClozeSegment::Blank("cat".to_string()));
    }

    #[test]
    fn test_render_blanks() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![
                ClozeSegment::Text("Hello ".to_string()),
                ClozeSegment::Blank("world".to_string()),
            ])
            .build();
        assert_eq!(cloze.render_blanks(), "Hello ___");
    }
}
```

---

## Test Utilities

Create helper functions in test modules for common setup:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn test_word() -> Word {
        Word::builder()
            .content("test")
            .build()
    }

    fn test_meaning(word_id: WordId) -> Meaning {
        Meaning::builder()
            .word_id(word_id)
            .definition("A test meaning")
            .pos(PartOfSpeech::Noun)
            .build()
    }

    #[test]
    fn test_word_with_meaning() {
        let word = test_word();
        let meaning = test_meaning(word.id);
        assert_eq!(meaning.word_id, word.id);
    }
}
```

---

## Integration Tests

Create integration tests in `tests/` directory at crate root:

```rust
// tests/registry_integration.rs
use clozer::models::{Word, Meaning};
use clozer::registry::WordRegistry;

#[test]
fn test_word_registry_crud() {
    let mut registry = WordRegistry::new();
    
    let word = Word::builder()
        .content("hello")
        .build();
    
    registry.add(word.clone());
    assert!(registry.exists(word.id));
    
    let retrieved = registry.get(word.id);
    assert_eq!(retrieved.content, "hello");
    
    registry.delete(word.id);
    assert!(!registry.exists(word.id));
}
```

---

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test --lib models::cloze

# Run tests with output
cargo test -- --nocapture

# Run tests with trace logging
RUST_LOG=trace cargo test
```

---

## Testing UI Components

UI components are harder to test directly. Focus on testing the state/logic:

```rust
// Test state management logic
#[test]
fn test_selection_toggle() {
    let mut selection = SelectionState::default();
    let item_id = Uuid::new_v4();
    
    assert!(!selection.is_selected(item_id));
    
    selection.toggle(item_id);
    assert!(selection.is_selected(item_id));
    
    selection.toggle(item_id);
    assert!(!selection.is_selected(item_id));
}

#[test]
fn test_selection_select_all() {
    let mut selection = SelectionState::default();
    let ids = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    
    selection.select_all(ids.clone());
    assert_eq!(selection.count(), 3);
    
    selection.clear();
    assert_eq!(selection.count(), 0);
}
```

---

## Mocking

For testing components that depend on external services (like LLM generation), use trait-based design:

```rust
// Define trait for testability
pub trait ClozeGenerator: Send + Sync {
    fn generate(&self, meaning: &Meaning) -> Result<Cloze, GenerateError>;
}

// In tests, use a mock implementation
#[cfg(test)]
mod mock {
    use super::*;

    pub struct MockGenerator;

    impl ClozeGenerator for MockGenerator {
        fn generate(&self, meaning: &Meaning) -> Result<Cloze, GenerateError> {
            Ok(Cloze::builder()
                .meaning_id(meaning.id)
                .segments(vec![ClozeSegment::Text("Test cloze".to_string())])
                .build())
        }
    }
}
```

---

## Related Rules

- [Dev Models](./dev-models.md) - Data structures
- [Dev Registry](./dev-registry.md) - In-memory storage
- [Dev UI](./dev-ui.md) - UI patterns
- [Dev Logging](./dev-logging.md) - Tracing for debugging tests
