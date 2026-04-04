# Logging

**Summary**: Structured logging with `tracing` for observability.

**Why**: Consistent log levels and structured fields enable effective debugging.

---

## Log Levels

| Level | When to Use |
|-------|-------------|
| `trace` | DTO conversions, low-level data flow |
| `debug` | Entity CRUD, data loading, state changes |
| `info` | Startup, shutdown, queue processing |
| `warn` | Config parse failures (using defaults), partial persistence failures |
| `error` | Database errors, save/delete failures |

---

## Structured Fields

Place fields before the message. Use `%` for Display, `?` for Debug.

```rust
tracing::error!(word_id = %id, error = %e, "Failed to save word");
tracing::debug!(count = loaded, "Loaded words from database");
tracing::debug!(elapsed_ms = elapsed, "LLM request completed");
tracing::trace!(?segment, "ClozeSegment -> ClozeSegmentDto");
```

**Don't** embed values in message strings: `"Loaded {} words"` → `count = loaded, "Loaded words"`

---

## Message Style

- Lowercase first letter
- Action in progress: present tense ("Creating word")
- Action completed: past tense ("Loaded words", "Saved word")
- Failure: present tense ("Failed to save word")

---

## Configuration Priority

| Priority | Source |
|----------|--------|
| 1 | CLI `--log-level` |
| 2 | `CLOZER_LOG_LEVEL` env var |
| 3 | `.clozer.toml` config |
| 4 | Default: `info` |

---

## Error Handling Pattern

```rust
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
