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
| **ProviderType** | Enum for LLM provider types (OpenAI, Anthropic, DeepSeek, Gemini, Ollama, Perplexity, XAI) |
| **QueueItem** | An item in the generation queue |
| **AppConfig** | Unified configuration with CLI, env, file, and defaults priority |

---

## Architecture Overview

The application follows Clean Architecture with five distinct layers:

| Layer | Purpose | Location |
|-------|---------|----------|
| **Models** | Pure data structures | `src/models/` |
| **Registry** | In-memory CRUD + indexes | `src/registry/` |
| **Persistence** | Database storage | `src/persistence/` |
| **State** | Business logic | `src/state/` |
| **UI** | Iced views | `src/ui/` |

### Key Architectural Decisions

- **Single-window** application via `iced::application`
- **Persistence**: In-memory registries sync with redb database
- **Dirty tracking**: `dirty_ids: BTreeSet<Uuid>` enables efficient flush of only modified entities
- **AI Integration**: LLM cloze generation via rig-core + reqwest
- **Tags**: Hierarchical (parent-child), associated with Meanings (not Words)
- **Selection**: Uses `HashSet<Uuid>` for O(1) operations
- **Assets**: Embedded via `include_dir!` macro for single-binary distribution

---

## Configuration Priority

| Priority | Source | Examples |
|----------|--------|----------|
| 1 (highest) | CLI arguments | `--data-dir`, `--config-file`, `--log-level` |
| 2 | Environment variables | `CLOZER_DATA_DIR`, `CLOZER_LOG_LEVEL` |
| 3 | Config file | `.clozer.toml` |
| 4 (lowest) | Defaults | Platform paths via `dirs`, log level `info` |

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

## Related Rules

- [Architecture](./arch-layers.md) - Layer responsibilities
- [Quick Start](./quick-start.md) - Build and run commands
- [Development Models](./dev-models.md) - Data structure patterns
