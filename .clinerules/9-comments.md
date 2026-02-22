# Comment Style Guide

**Summary**: Rust documentation conventions following RFC 1574 and RFC 1713.

**Why**: Ensures consistent, readable documentation across the codebase.

---

## Quick Reference

| Type | Syntax | Use For |
|------|--------|---------|
| Doc | `///` | Public API documentation |
| Line | `//` | Code organization |
| Module | `//!` | mod.rs file documentation |

---

## Doc Comments (///)

Use `///` for documenting public API items (structs, enums, functions, methods). These are machine-readable and processed by rustdoc.

### Supported Sections

| Section | Purpose |
|---------|---------|
| `# Parameters` | List function parameters |
| `# Returns` | Describe return value |
| `# Type parameters` | List generic type parameters |
| `# Lifetimes` | List lifetime parameters |
| `# Examples` | Provide usage examples |
| `# Panics` | Document panic conditions |
| `# Errors` | Document error conditions |
| `# Safety` | Document safety requirements |
| `# See also` | Link to related items |

### List Syntax

All parameter/field lists must use this format:

```rust
/// - `name`: Description (Markdown, can be multi-line)
```

### Example

```rust
/// Fooify a `Foo` with a label
///
/// # Parameters
///
/// - `label`: A string labelling the foo
/// - `magic`: A `Foo` that will be labeled
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: A `Bar` that is the labeled foo
/// - `Err`: Returns the number of gravely appalled people
///
/// # Examples
///
/// ```rust
/// assert_eq!(fooify("lorem", foo).label(), "lorem")
/// ```
///
/// # See also
///
/// - [`Bar::from_foo`]
fn fooify<'a, T>(label: T, magic: Foo<'a>) -> Result<Bar<'a>, i32> { ... }
```

---

## Line Comments (//)

Use `//` for code organization and inline explanations.

### Section Markers

Organize code into logical sections:

```rust
// Word operations
match message { ... }

// CRUD operations
pub fn insert(&mut self, word: Word) { ... }

// Helpers
pub fn count(&self) -> usize { ... }
```

### Inline Comments

Only explain non-obvious logic:

```rust
// Good - explains non-obvious logic
self.by_tag
    .entry(tag_id)
    .or_insert_with(BTreeSet::new)
    .insert(meaning_id);
```

### Rules

1. **Attach comments directly to code** - Comments should not be followed by a blank line before the code they describe.

   ```rust
   // Good - comment directly attached
   let greeting = Meaning::builder().build();

   // Bad - orphan comment (blank line between)
   // Create a meaning for "Hello"

   let greeting = Meaning::builder().build();
   ```

2. Use plain `//` only - No `//=`, `//===`, or other decorations

3. Space after `//` - Write `// Word operations`, not `//Word operations`

4. Capitalize first letter - `// Delete all clozes`, not `// delete all clozes`

5. Keep comments brief - Prefer self-explanatory code over verbose comments

---

## Module Comments (//!)

Use `//!` for module-level documentation in `mod.rs` files.

```rust
//! Data access layer with secondary indexes.
//!
//! Manages in-memory storage with BTreeMap for deterministic iteration.
```

---

## Related Rules

- [Architecture](./1-architecture.md) - Module structure
- [Models](./2-models.md) - Data structures
