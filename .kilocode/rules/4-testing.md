# Testing

**Summary**: Testing patterns - unit tests, integration tests, and mocking.

**Why**: Ensures code quality with consistent testing approaches.

---

## Test Organization

Co-locate tests using `#[cfg(test)]` module pattern:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_sentence() {
        let segments = Cloze::parse_from_sentence("The [cat] sat on the [mat]");
        assert_eq!(segments.len(), 5);
    }
}
```

---

## Test Utilities

Create helper functions for common setup:

```rust
#[cfg(test)]
mod tests {
    fn test_word() -> Word {
        Word::builder().content("test").build()
    }

    #[test]
    fn test_word_with_meaning() {
        let word = test_word();
        assert!(!word.meaning_ids.is_empty());
    }
}
```

---

## Integration Tests

Place in `tests/` directory at crate root:

```rust
// tests/registry_integration.rs
use clozer::models::Word;
use clozer::registry::WordRegistry;

#[test]
fn test_word_registry_crud() {
    let mut registry = WordRegistry::new();
    let word = Word::builder().content("hello").build();
    registry.add(word.clone());
    assert!(registry.exists(word.id));
}
```

---

## Testing State Logic

UI state is testable directly - focus on state/logic testing:

```rust
#[test]
fn test_selection_toggle() {
    let mut selection = SelectionState::default();
    let id = Uuid::new_v4();
    
    assert!(!selection.is_selected(id));
    selection.toggle(id);
    assert!(selection.is_selected(id));
    selection.toggle(id);
    assert!(!selection.is_selected(id));
}
```

---

## Mocking External Services

Use trait-based design for testable dependencies:

```rust
pub trait ClozeGenerator: Send + Sync {
    fn generate(&self, meaning: &Meaning) -> Result<Cloze, GenerateError>;
}

#[cfg(test)]
impl ClozeGenerator for MockGenerator {
    fn generate(&self, meaning: &Meaning) -> Result<Cloze, GenerateError> {
        Ok(Cloze::builder()
            .meaning_id(meaning.id)
            .segments(vec![ClozeSegment::Text("Test".to_string())])
            .build())
    }
}
```

---

## Running Tests

```bash
cargo test                    # All tests
cargo test --lib models::cloze  # Specific module
cargo test -- --nocapture      # Show output
RUST_LOG=trace cargo test      # With tracing
```
