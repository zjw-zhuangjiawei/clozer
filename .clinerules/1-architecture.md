# Architecture

**Summary**: Clean Architecture with clear separation of concerns - models, registry, persistence, state, and UI layers.

**Why**: Each layer has distinct responsibilities, making the codebase testable and maintainable.

---

## Module Structure

```
src/
├── main.rs           # Entry point
├── lib.rs            # Module exports (App, Message, submodules)
├── app.rs            # App struct, new(), title(), update(), view()
├── message.rs        # Message enum for Elm-like architecture
├── assets.rs         # Embedded assets (SVG icons) via include_dir
├── config/           # Configuration
│   ├── cli.rs        # CLI arguments (--data-dir, --config-file, --log-level)
│   ├── env.rs        # Environment variables (CLOZER_*)
│   ├── constants.rs  # Default paths
│   ├── file/         # File configuration (.clozer.toml)
│   │   ├── ai.rs     # AI provider configuration
│   │   ├── general.rs
│   │   └── mod.rs
│   └── mod.rs        # AppConfig with priority: CLI > env > file > defaults
├── models/           # Pure data structures
│   ├── word.rs       # Word with optional language
│   ├── meaning.rs    # Meaning with PartOfSpeech and CefrLevel
│   ├── cloze.rs      # Cloze with segment parsing
│   ├── tag.rs        # Hierarchical tags
│   ├── model.rs
│   ├── provider.rs
│   └── mod.rs
├── registry/         # Data access layer with secondary indexes + dirty tracking
│   ├── word.rs
│   ├── meaning.rs
│   ├── cloze.rs
│   ├── tag.rs
│   ├── model.rs
│   ├── provider.rs
│   ├── queue.rs
│   └── mod.rs
├── persistence/      # Persistent storage (redb)
│   ├── db/           # Database operations per entity
│   │   ├── core.rs   # Table definitions, serialization helpers
│   │   ├── words.rs
│   │   ├── meanings.rs
│   │   ├── clozes.rs
│   │   ├── tags.rs
│   │   └── mod.rs
│   ├── dto/          # Data Transfer Objects for serialization
│   │   ├── word.rs
│   │   ├── meaning.rs
│   │   ├── cloze.rs
│   │   ├── tag.rs
│   │   └── mod.rs
│   └── mod.rs
├── state/            # State coordination and message handling
│   ├── generator.rs  # GeneratorState for LLM integration
│   ├── model.rs      # Model (data + business logic)
│   ├── queue.rs      # QueueState for generation queue
│   └── mod.rs        # AppState (orchestrator)
└── ui/               # Iced UI components
    ├── mod.rs
    ├── app.rs        # App-level view/update functions
    ├── message.rs    # MainWindowMessage enum
    ├── state.rs      # MainWindowState
    ├── nav.rs        # Navigation items
    ├── theme.rs      # Theme definitions (AppTheme, ThemeColors)
    ├── components/   # Reusable UI components
    │   ├── checkbox.rs
    │   └── mod.rs
    ├── queue/        # Queue view sub-module
    │   ├── message.rs
    │   ├── state.rs
    │   ├── update.rs
    │   └── view.rs
    ├── settings/     # Settings view sub-module
    │   ├── message.rs
    │   ├── state.rs
    │   ├── update.rs
    │   └── view.rs
    └── words/        # Words view sub-module
        ├── message.rs
        ├── state.rs
        ├── update.rs
        ├── view.rs
        └── detail_view.rs
```

## Per-Entity Pattern

Each entity follows this pattern across layers:

| Layer | Purpose | Example Files |
|-------|---------|---------------|
| `models/` | Struct definition | `word.rs`, `meaning.rs` |
| `registry/` | In-memory CRUD + indexes | `word.rs`, `meaning.rs` |
| `persistence/db/` | Disk storage ops | `words.rs`, `meanings.rs` |
| `persistence/dto/` | Serialization DTOs | `word.rs`, `meaning.rs` |

## Entities

| Entity | Description |
|--------|-------------|
| `Word` | A word to learn, has unique UUID, optional language, and meaning associations |
| `Meaning` | Definition with PartOfSpeech, CefrLevel, and tag associations |
| `Cloze` | Fill-in-the-blank sentence with segments, derived from meaning |
| `Tag` | Hierarchical tag/category with parent-child relationships |
| `Model` | LLM model configuration |
| `Provider` | LLM provider for generating cloze sentences |
| `QueueItem` | An item in the generation queue |

## Layer Responsibilities

### 1. Models (`src/models/`)

Pure data structures with typed_builder, no business logic.

- `Word`: Content string, UUID, meaning associations, optional language
- `Meaning`: Definition, PartOfSpeech, CefrLevel, tag associations
- `Cloze`: Fill-in-the-blank sentence with segments, source meaning reference
- `Tag`: Name, UUID, parent-child hierarchy

### 2. Registry (`src/registry/`)

Data access layer. Manages in-memory storage with `BTreeMap` (ordered, deterministic iteration) and secondary indexes.

> **Note**: Uses `BTreeMap` and `BTreeSet` instead of `HashMap`/`HashSet` for deterministic ordering and iteration.

- CRUD operations for entities
- Secondary indexes for efficient lookups (e.g., `by_tag`, `by_word`)
- Iterator methods (`iter()`, `iter_by_tag()`, `iter_by_word()`)
- Dirty tracking for efficient persistence (`dirty_ids: BTreeSet<Uuid>`)

### 3. Persistence (`src/persistence/`)

Persistent storage layer using redb database.

- `Db`: Database connection and operations
- `persistence/db/`: Table definitions and CRUD operations
- `persistence/dto/`: Data Transfer Objects for serialization
- Serialization via `rmp-serde` (MessagePack)
- Syncs in-memory registries with disk

### 4. State (`src/state/`)

Coordinates registries and services. Update logic is in per-window `update()` modules.

- `Model`: Holds all registries + database connection + generator
- `AppState`: Orchestrator that holds `Model`
- `GeneratorState`: LLM generator for cloze generation
- `QueueState`: Manages generation queue

### 5. UI (`src/ui/`)

Iced views and messages. No business logic.

- View functions take all needed state as parameters
- Return `Element<'_, ParentMessage>`
- Use `.map()` to transform child messages to parent messages

## Development Workflow

1. Add data → `models/`
2. Add storage + indexes → `registry/` and `persistence/`
3. Add operations → `state/`
4. Add views → `ui/`

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
