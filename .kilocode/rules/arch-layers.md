# Architecture Layers

**Summary**: Clean Architecture with clear separation of concerns - models, registry, persistence, state, and UI layers.

**Why**: Each layer has distinct responsibilities, making the codebase testable and maintainable.

---

## Layer Overview

The application follows Clean Architecture with five distinct layers:

| Layer | Purpose | Location |
|-------|---------|----------|
| **Models** | Pure data structures | `src/models/` |
| **Registry** | In-memory CRUD + indexes + dirty tracking | `src/registry/` |
| **Persistence** | Database storage | `src/persistence/` |
| **State** | Business logic coordination | `src/state/` |
| **UI** | Iced views and messages | `src/ui/` |

---

## Layer Responsibilities

### 1. Models (`src/models/`)

Pure data structures with typed_builder, no business logic.

- `Word`: Content string, UUID, meaning associations, optional language
- `Meaning`: Definition, PartOfSpeech, CefrLevel, tag associations
- `Cloze`: Fill-in-the-blank sentence with segments, source meaning reference
- `Tag`: Name, UUID, parent-child hierarchy
- `Model`: LLM model configuration
- `Provider`: LLM provider configuration
- `types.rs`: Newtype ID types (WordId, MeaningId, TagId, ClozeId, ProviderId, ModelId) for enhanced type safety

### 2. Registry (`src/registry/`)

Data access layer. Manages in-memory storage with `BTreeMap` (ordered, deterministic iteration) and secondary indexes.

- CRUD operations for entities
- Secondary indexes for efficient lookups (e.g., `by_tag`, `by_word`)
- Iterator methods (`iter()`, `iter_by_tag()`, `iter_by_word()`)
- Dirty tracking for efficient persistence (`dirty_ids: BTreeSet<Uuid>`)

> **Note**: Uses `BTreeMap` and `BTreeSet` instead of `HashMap`/`HashSet` for deterministic ordering and iteration.

### 3. Persistence (`src/persistence/`)

Persistent storage layer using redb database.

- `Db`: Database connection and operations
- `persistence/db/`: Table definitions and CRUD operations
- `persistence/dto/`: Data Transfer Objects for serialization
- Serialization via `rmp-serde` (MessagePack)
- Syncs in-memory registries with disk

### 4. State (`src/state/`)

Coordinates registries and services.

- `Model`: Holds all registries + database connection + generator
- `AppState`: Orchestrator that holds `Model`
- `GeneratorState`: LLM generator for cloze generation
- `QueueState`: Manages generation queue

### 5. UI (`src/ui/`)

Iced views and messages. No business logic.

- View functions take all needed state as parameters
- Return `Element<'_, ParentMessage>`
- Use `.map()` to transform child messages to parent messages

---

## Per-Entity Pattern

Each entity follows this pattern across layers:

| Layer | Purpose | Example Files |
|-------|---------|---------------|
| `models/` | Struct definition | `word.rs`, `meaning.rs` |
| `registry/` | In-memory CRUD + indexes | `word.rs`, `meaning.rs` |
| `persistence/db/` | Disk storage ops | `words.rs`, `meanings.rs` |
| `persistence/dto/` | Serialization DTOs | `word.rs`, `meaning.rs` |

---

## Development Workflow

1. Add data → `models/`
2. Add storage + indexes → `registry/` and `persistence/`
3. Add operations → `state/`
4. Add views → `ui/`

---

## Logging

Uses `tracing` and `tracing-subscriber` with target-based filtering.

```rust
impl LogLevel {
    pub const fn into_tracing_level(self) -> Level {
        match self {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}
```

Configure with `--log-level` CLI arg, `CLOZER_LOG_LEVEL` env var, or config file.

---

## Startup and Shutdown

```rust
// On app startup
impl App {
    pub fn new(config: AppConfig) -> (Self, Task<Message>) {
        // Load existing data from database
        let data = DataState::load_all(&config);
        // Start tracing subscriber
    }
}

// On app shutdown
impl App {
    pub fn on_exit(&mut self) {
        // Flush dirty entities to disk
        self.data.flush_all();
        // Save config to file
    }
}
```

---

## Related Rules

- [Overview](./overview.md) - Project overview
- [Architecture Modules](./arch-modules.md) - Module structure details
- [Dev Models](./dev-models.md) - Data structure patterns
- [Dev Registry](./dev-registry.md) - In-memory storage
- [Dev Persistence](./dev-persistence.md) - Database patterns
- [Dev UI](./dev-ui.md) - Iced UI patterns
- [Dev Error Handling](./dev-error-handling.md) - Error handling patterns
- [Dev Testing](./dev-testing.md) - Testing patterns
