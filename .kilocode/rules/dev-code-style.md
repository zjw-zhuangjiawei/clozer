# Development: Code Style

**Summary**: Rust code formatting conventions for this project.

**Why**: Ensures consistent, readable code that follows Rust idioms and maximizes maintainability.

---

## Section Separators

**Summary**: Use blank lines for code separation instead of heavy section title comments.

**Why**: Heavy section titles (`// ====================...`) are verbose and not idiomatic Rust. Rust uses blank lines and doc comments (`///`) to separate code sections.

### ✅ Do This

```rust
/// Handle selection-related messages.
pub fn selection(...) {
    // ...
}

/// Handle action-related messages.
pub fn action(...) {
    // ...
}
```

### ❌ Don't Do This

```rust
// ============================================================================
// Selection Handler
// ============================================================================

/// Handle selection-related messages.
pub fn selection(...) {
    // ...
}

// ============================================================================
// Action Handler
// ============================================================================

/// Handle action-related messages.
pub fn action(...) {
    // ...
}
```

**Why**: The heavy separators add visual noise without providing value beyond what blank lines and doc comments already convey. The Rust compiler and rustfmt handle code formatting automatically.

---

## Related Rules

- [Dev: Logging](./dev-logging.md) - Tracing patterns
- [Dev: Models](./dev-models.md) - Data structures
- [Dev: UI](./dev-ui.md) - UI patterns
