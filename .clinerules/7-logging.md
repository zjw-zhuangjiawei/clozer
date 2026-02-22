# Logging

**Summary**: Structured logging with `tracing` and `tracing-subscriber` for observability.

**Why**: Provides consistent logging patterns across all layers with proper log levels and field formatting.

---

## Log Levels

| Level | Usage | When to Use |
|-------|-------|-------------|
| `trace` | Detailed internal operations | DTO conversions, low-level data flow, verbose diagnostics |
| `debug` | Operational details | Entity CRUD operations, data loading, internal state changes |
| `info` | Significant lifecycle events | Startup, shutdown, configuration loaded, queue processing |
| `warn` | Recoverable issues | Failed to parse config (using defaults), partial persistence failures |
| `error` | Failures requiring attention | Database errors, failed saves/deletes, flush failures |

## Structured Fields

Use structured fields for contextual information. Place fields before the message string.

```rust
// Good - structured fields for context
tracing::error!(error = %e, source = "word_registry", "Failed to load words from database");
tracing::debug!(count = loaded, "Loaded words from database");
tracing::debug!(elapsed_ms = elapsed, "LLM request completed");
tracing::error!(word_id = %id, error = %e, "Failed to save word");

// Bad - embedded in message string
tracing::error!("Failed to load words from database: {}", e);
tracing::debug!("Loaded {} words from database", loaded);
```

### Field Formatting

| Syntax | Use For |
|--------|---------|
| `key = value` | Regular values (numbers, strings) |
| `key = %value` | Display trait (`%` shorthand for `Display`) |
| `key = ?value` | Debug trait (`?` shorthand for `Debug`) |

```rust
// Display for user-friendly output
tracing::error!(word_id = %id, error = %e, "Failed to save word");

// Debug for complex types
tracing::trace!(?segment, "ClozeSegment -> ClozeSegmentDto");
```

## Message Style

### Tense

| Situation | Tense | Example |
|-----------|-------|---------|
| Action in progress | Present | "Creating word", "Loading configuration" |
| Action completed | Past | "Loaded 5 words", "Saved word" |
| Failure | Present | "Failed to save word" |

### Format

- Lowercase first letter (except proper nouns)
- Concise, descriptive messages
- Include entity type when relevant

```rust
// Good
tracing::debug!("Creating word: {} (id={})", word.content, word.id);
tracing::info!("Configuration loaded: data_dir={:?}, log_level={:?}", data_dir, log_level);
tracing::debug!("Loaded {} words from database", items.len());

// Bad
tracing::debug!("Creating Word...");  // Missing context
tracing::info!("Configuration Loaded Successfully!");  // Unnecessary capitalization/punctuation
```

## Layer-Specific Patterns

### Registry Layer (`src/registry/`)

Log load operations and persistence errors.

```rust
// Loading data
impl WordRegistry {
    pub fn load_all(&mut self, db: &Db) {
        let count = self.words.len();
        match db.iter_words() {
            Ok(items) => {
                for dto in items {
                    let word = Word::from(dto);
                    self.words.insert(word.id, word);
                }
            }
            Err(e) => {
                tracing::error!(error = %e, source = "word_registry", "Failed to load words from database");
            }
        }
        let loaded = self.words.len() - count;
        tracing::debug!(count = loaded, "Loaded words from database");
    }
}
```

### Persistence Layer (`src/persistence/db/`)

Log save, delete, and load operations per entity.

```rust
// Database operations
pub fn save(&self, id: Uuid, word: &Word) -> Result<()> {
    // ... save logic ...
    tracing::debug!("Saved word {}", id);
    Ok(())
}

pub fn delete(&self, id: Uuid) -> Result<()> {
    // ... delete logic ...
    tracing::debug!("Deleted word {}", id);
    Ok(())
}
```

### DTO Layer (`src/persistence/dto/`)

Use `trace` level for conversion logging.

```rust
impl From<ClozeSegment> for ClozeSegmentDto {
    fn from(segment: ClozeSegment) -> Self {
        let result = ClozeSegmentDto { /* ... */ };
        tracing::trace!(?segment, "ClozeSegment -> ClozeSegmentDto");
        result
    }
}
```

### State Layer (`src/state/`)

Log significant state changes and operations.

```rust
// Queue processing
pub fn process(&mut self, /* ... */) -> Task<Message> {
    let count = items.len();
    tracing::info!("Processing queue: {} pending items", count);
    // ...
}

// LLM operations
tracing::debug!(elapsed_ms = elapsed, "LLM request completed");
```

### Configuration (`src/config/`)

Log configuration loading and saving.

```rust
// Loading
tracing::debug!("Loading configuration...");
tracing::info!("Loaded configuration from file: {:?}", config_file);
tracing::info!("Configuration loaded: data_dir={:?}, log_level={:?}", data_dir, log_level);

// Saving
tracing::debug!("Saving configuration to file: {:?}", self.config_file);
tracing::info!("Configuration saved to: {:?}", self.config_file);
```

### App Lifecycle (`src/app.rs`, `src/main.rs`)

Log startup and shutdown events.

```rust
// Startup
tracing::info!(
    target: "clozer::startup",
    "Starting Clozer v{}",
    env!("CARGO_PKG_VERSION")
);
tracing::debug!("Initializing database at {:?}", db_path);
tracing::debug!("Loading data from database");

// Shutdown
tracing::debug!("Flushing dirty data on shutdown");
tracing::info!("Clozer shutting down");
```

## Error Handling Pattern

Log errors at appropriate levels with context, then handle or propagate.

```rust
// Registry persistence errors
let mut errors = 0;
for (id, word) in self.words.iter() {
    if let Err(e) = db.words().save(*id, word) {
        tracing::error!(word_id = %id, error = %e, "Failed to save word");
        errors += 1;
    }
}
if errors > 0 {
    tracing::warn!(errors = errors, "Some words failed to persist");
}
```

## Configuration

Configure log level via:

| Priority | Source | Example |
|----------|--------|---------|
| 1 (highest) | CLI argument | `--log-level debug` |
| 2 | Environment variable | `CLOZER_LOG_LEVEL=debug` |
| 3 | Config file | `.clozer.toml` |
| 4 (lowest) | Default | `info` |

```toml
# .clozer.toml
[general]
log_level = "debug"
```

### LogLevel Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub const DEFAULT: Self = LogLevel::Info;
}
```
