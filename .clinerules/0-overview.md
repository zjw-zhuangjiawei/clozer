# Clozer Project Overview

**Summary**: Desktop application for creating cloze deletion cards using Rust and Iced GUI framework.

**Why**: Provides context for all development decisions. Understanding the project structure helps navigate codebase efficiently.

---

## Project Identity

| Attribute | Value |
|-----------|-------|
| **Name** | Clozer |
| **Type** | Desktop application |
| **Language** | Rust |
| **GUI Framework** | Iced 0.14.0 |
| **Edition** | 2024 |

---

## Key Entities

| Entity | Description |
|--------|-------------|
| **Word** | A word to learn, has unique UUID, meaning associations, and optional language |
| **Meaning** | Definition of a word with PartOfSpeech, CEFR level, and tag associations |
| **Cloze** | A fill-in-the-blank sentence with segments, derived from a meaning (AI-generated only) |
| **Tag** | A hierarchical tag/category for organizing meanings (supports parent-child relationships) |
| **Model** | LLM model configuration for AI-powered cloze generation |
| **Provider** | LLM provider for generating cloze sentences |
| **QueueItem** | An item in the generation queue |
| **AppConfig** | Unified configuration with CLI, env, file, and defaults priority |

---

## Dependencies

```toml
[package]
name = "clozer"
edition = "2024"

[dependencies]
# GUI
iced = { version = "0.14.0", features = ["tokio", "svg"] }
rfd = "0.17"

# Core
uuid = { version = "1", features = ["v4", "serde"] }
rand = "0.10.0"
typed-builder = "0.23.2"
derive_more = { version = "2.1.1", features = ["full"] }
thiserror = "2.0.18"
either = "1.15.0"

# Enums
strum = { version = "0.27.2", features = ["derive"] }

# Persistence
redb = "3.1.0"
rmp-serde = "1.3"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0"

# AI/LLM Integration
rig-core = "0.31.0"
reqwest = "0.13.2"
tokio = { version = "1.49.0", features = ["full"] }

# Configuration
clap = { version = "4.5", features = ["derive"] }
dirs = "6.0.0"
envy = "0.4.2"
toml = "0.9.8"

# Utilities
langtag = { version = "1.1.0", features = ["serde"] }
fancy-regex = "0.17.0"
once_cell = "1.21.3"
include_dir = { version = "0.7.4", features = ["glob", "metadata"] }
tracing = "0.1.44"
tracing-subscriber = "0.3.22"

# Platform-specific
[target.'cfg(windows)'.dependencies]
windows = { version = "0.62", features = ["Win32_UI_WindowsAndMessaging"] }
```

---

## Source Structure

```
src/
├── main.rs           # Entry point
├── lib.rs            # Module exports
├── app.rs            # App struct: new(), title(), update(), view()
├── message.rs        # Message enum (Elm-like)
├── assets.rs         # Embedded SVG icons via include_dir!
│
├── config/           # Configuration (CLI > env > file > defaults)
│   ├── cli.rs        # --data-dir, --config-file, --log-level
│   ├── env.rs        # CLOZER_* variables
│   ├── constants.rs  # Default paths
│   ├── file/         # .clozer.toml
│   │   ├── ai.rs     # AI provider config
│   │   ├── general.rs
│   │   └── mod.rs
│   └── mod.rs        # AppConfig
│
├── models/           # Pure data structures
│   ├── word.rs       # Word + optional language
│   ├── meaning.rs    # Meaning + PartOfSpeech + CefrLevel
│   ├── cloze.rs      # Cloze + segment parsing
│   ├── tag.rs        # Hierarchical tags
│   ├── model.rs      # LLM model config
│   ├── provider.rs   # LLM provider config
│   └── mod.rs
│
├── registry/         # In-memory CRUD + indexes + dirty tracking
│   ├── word.rs
│   ├── meaning.rs
│   ├── cloze.rs
│   ├── tag.rs
│   ├── model.rs
│   ├── provider.rs
│   ├── queue.rs
│   └── mod.rs
│
├── persistence/      # redb database
│   ├── db/           # Table operations
│   │   ├── core.rs   # Schema + helpers
│   │   ├── words.rs
│   │   ├── meanings.rs
│   │   ├── clozes.rs
│   │   └── tags.rs
│   ├── dto/          # Serialization DTOs
│   │   ├── word.rs
│   │   ├── meaning.rs
│   │   ├── cloze.rs
│   │   └── tag.rs
│   └── mod.rs
│
├── state/            # Business logic layer
│   ├── generator.rs  # LLM generator
│   ├── model.rs      # Data + db connection
│   ├── queue.rs      # Queue state
│   └── mod.rs        # AppState (orchestrator)
│
└── ui/               # Iced views
    ├── mod.rs
    ├── app.rs        # App-level view/update
    ├── message.rs    # MainWindowMessage
    ├── state.rs      # MainWindowState
    ├── nav.rs        # NavItem enum
    ├── theme.rs      # ThemeColors
    ├── components/   # Reusable components
    │   └── checkbox.rs
    ├── queue/        # Queue panel
    ├── settings/     # Settings panel
    └── words/        # Words panel + detail

# Binaries
inspect-db/main.rs        # Database inspection tool
create-sample-db/main.rs  # Sample data generator
```

---

## Configuration Priority

| Priority | Source | Examples |
|----------|--------|----------|
| 1 (highest) | CLI arguments | `--data-dir`, `--config-file`, `--log-level` |
| 2 | Environment variables | `CLOZER_DATA_DIR`, `CLOZER_LOG_LEVEL` |
| 3 | Config file | `.clozer.toml` |
| 4 (lowest) | Defaults | Platform paths via `dirs`, log level `info` |

---

## Key Architectural Notes

- **Single-window** application via `iced::application`
- **Persistence**: In-memory registries sync with redb database
- **Dirty tracking**: `dirty_ids: BTreeSet<Uuid>` enables efficient flush of only modified entities
- **AI Integration**: LLM cloze generation via rig-core + reqwest
- **Tags**: Hierarchical (parent-child), associated with Meanings (not Words)
- **Selection**: Uses `HashSet<Uuid>` for O(1) operations
- **Assets**: Embedded via `include_dir!` macro for single-binary distribution

---

## Rules Files

| File | Description |
|------|-------------|
| `0-overview.md` | This file |
| `1-architecture.md` | Module structure and layer responsibilities |
| `2-models.md` | Model definitions with typed_builder |
| `3-registry.md` | Registry CRUD, iterators, dirty tracking |
| `4-persistence.md` | DTO pattern, serialization |
| `5-ui-patterns.md` | Iced UI patterns |
| `6-api-design.md` | Builder, strum enums, traits |
| `7-logging.md` | Tracing patterns |
| `8-git-conventions.md` | Commit conventions |
| `9-comments.md` | Documentation style |
| `10-common-operations.md` | Build/run commands |

---

## Related Rules

- [Architecture](./1-architecture.md) - Layer responsibilities
- [Models](./2-models.md) - Data structure patterns
- [Registry](./3-registry.md) - Data access patterns
- [UI Patterns](./5-ui-patterns.md) - Iced patterns
