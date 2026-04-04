# Style Guide

**Summary**: Rust naming conventions and code formatting rules.

**Why**: Consistent code style reduces cognitive load and improves readability.

---

## Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Variables | camelCase | `user_name`, `is_active` |
| Structs/Enums | PascalCase | `WordRegistry`, `PartOfSpeech` |
| Constants | UPPER_SNAKE | `MAX_RETRIES`, `DEFAULT_TIMEOUT` |
| Files (structs) | snake_case | `word_registry.rs` |
| Files (utility) | snake_case | `parse_json.rs` |
| ID newtypes | PascalCase | `WordId`, `MeaningId` |

---

## Section Separators

Use blank lines and doc comments (`///`) for code separation.

```rust
/// Handle selection-related messages.
pub fn selection(...) { ... }

/// Handle action-related messages.
pub fn action(...) { ... }
```

**Don't** use heavy separators like `// ===...===`.

---

## Formatting

- Run `cargo fmt` before committing
- Use `cargo clippy` to catch anti-patterns
- Derives: `#[derive(Debug, Clone)]` for state, `#[derive(PartialEq, Eq)]` for comparisons
