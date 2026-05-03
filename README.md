# Clozer

**AI-powered cloze deletion flashcards for language learning.**

Clozer is a desktop application that helps language learners create **cloze deletion (fill-in-the-blank)** flashcards. Given a vocabulary word and its definition, Clozer uses LLMs (OpenAI, Anthropic, DeepSeek, Gemini, Ollama, and more) to automatically generate cloze sentences. All data is stored locally in an embedded database.

## Features

- **Word & Meaning Management** — Create, edit, and organize vocabulary with definitions, parts of speech, CEFR levels, and language tags (BCP 47).
- **Dictionary Integration** — Look up definitions from FreeDictionaryAPI to auto-fill meanings.
- **AI-Powered Cloze Generation** — Generate cloze sentences from multiple LLM providers: OpenAI, Anthropic, DeepSeek, Gemini, Ollama, Perplexity, xAI.
- **Queue System** — Batch-process meanings for cloze generation with status tracking (Pending → Processing → Completed/Failed).
- **Tag System** — Hierarchical tags for categorizing meanings; dedicated management panel.
- **Search & Filter** — Query-based search with AND/OR syntax, tag filters, POS filters, status filters, exclude syntax, and autocomplete.
- **Light & Dark Themes** — Perceptual color scales (OKLCH) with light and dark mode support.
- **Responsive Layout** — Adapts across 5 breakpoints (480px–1200px+); sidebar collapses to a bottom tab bar on narrow screens.
- **Local Persistence** — Embedded redb database with MessagePack serialization; data flushed on shutdown.
- **Export** — Plaintext export of words and meanings.

## Installation

### Prerequisites

- Rust 2024 edition (Rust 1.85+)

### Build from source

```bash
git clone https://github.com/your-username/clozer.git
cd clozer
cargo build --release
```

The binary will be at `target/release/clozer`.

### Run

```bash
./target/release/clozer
```

On the first run, Clozer creates a default config file and data directory at `~/.config/clozer/`.

## Configuration

Clozer uses a **three-tier priority system** (highest to lowest):

| Priority | Source | Example |
|----------|--------|---------|
| 1 | CLI arguments | `--theme dark` |
| 2 | Environment variables | `CLOZER_THEME=dark` |
| 3 | TOML config file | `~/.config/clozer/clozer.toml` |

### CLI

```
clozer [OPTIONS]

Options:
  -d, --data-dir <PATH>       Path to the data directory
  -c, --config-file <PATH>    Path to the config file
      --log-level <LEVEL>     Log level (trace, debug, info, warn, error)
      --theme <THEME>         UI theme (light, dark)
  -h, --help                  Print help
```

### Environment variables

- `CLOZER_DATA_DIR`
- `CLOZER_CONFIG_FILE`
- `CLOZER_LOG_LEVEL`
- `CLOZER_THEME`

### Config file (TOML)

```toml
[general]
data_dir = "./.clozer-data"
log_level = "debug"
theme = "light"

[ai]
selected_model_id = "<uuid>"

[[ai.providers]]
id = "<uuid>"
name = "DeepSeek"
provider_type = "deepseek"
api_key = "sk-..."
base_url = "https://api.deepseek.com"

[[ai.models]]
id = "<uuid>"
name = "DeepSeek Chat"
provider_id = "<provider-uuid>"
model_id = "deepseek-chat"
```

Providers and models can also be managed through the Settings panel in the UI.

## Search Query Syntax

The search bar in the Words panel supports a rich query syntax:

| Syntax | Example | Description |
|--------|---------|-------------|
| `text` | `hello` | Search word content or definitions |
| `#tag` | `#vocabulary` | Filter by tag |
| `-#tag` | `-#ignored` | Exclude by tag |
| `:pos` | `:noun` | Filter by part of speech |
| `-:pos` | `-:verb` | Exclude by part of speech |
| `is:status` | `is:pending` | Filter by status (`pending`, `done`, `cloze`, `plain`) |
| `-is:status` | `-is:done` | Exclude by status |
| `\|` | `hello \| world` | OR operator |
| `( )` | `(#tag1 \| #tag2) :noun` | Grouping |
| Space | `hello world` | Implicit AND |

**POS shortcuts**: `n` (noun), `v` (verb), `adj` (adjective), `adv` (adverb), `pron` (pronoun), `prep` (preposition), `conj` (conjunction), `interj` (interjection), `det` (determiner), `art` (article), `modal` (modal), `num` (numeral), `abbr` (abbreviation).

## Architecture

Clozer follows **The Elm Architecture** (via the [Iced](https://iced.rs/) GUI framework) — unidirectional data flow:

```
User Input → Message → Update → Model → View
```

- **Model** (`state::Model`) — holds all registries, database handle, generator, and config.
- **Message** (`message::Message`) — flat enum dispatched per panel (Words, Queue, Tags, Settings).
- **View** — renders UI from the current model state.
- **Update** — transforms state in response to messages.

### Project structure

```
src/
├── main.rs              # Entry point
├── lib.rs               # Library root
├── app.rs               # Iced Application trait implementation
├── message.rs           # Top-level Message enum
├── assets.rs            # Embedded SVG icons
├── config/              # CLI args, env vars, file config, constants
├── models/              # Domain types: Word, Meaning, Cloze, Tag, Provider, Model
├── persistence/         # redb database + DTOs
├── registry/            # In-memory registries with dirty tracking
├── state/               # Model, Generator, Queue processing
├── query/               # Search query parser and engine
├── dictionary/          # Dictionary API integration
└── ui/                  # All UI panels, widgets, theme, layout
    ├── words/           # Words panel (explorer + detail + managers)
    ├── tags/            # Tags panel
    ├── queue/           # Queue panel
    ├── settings/        # Settings panel
    ├── theme/           # Light/dark theme with OKLCH color scales
    └── widgets/         # Custom iced widget overrides
```

## Data Storage

All data is stored in a local [redb](https://github.com/cberner/redb) database (embedded key-value store, similar to SQLite but lower-level). Entities are serialized with [MessagePack](https://msgpack.org/) via `rmp-serde`. The database file is located in the configured data directory.

## Tech Stack

- **GUI**: [Iced](https://iced.rs/) 0.14 — cross-platform GUI for Rust
- **Database**: [redb](https://github.com/cberner/redb) 3.1 — embedded key-value store
- **LLM Client**: [rig-core](https://crates.io/crates/rig-core) 0.31 — supports OpenAI, Anthropic, DeepSeek, Gemini, Ollama, Perplexity, xAI
- **Serialization**: MessagePack via `rmp-serde`
- **Async Runtime**: Tokio
- **Color**: OKLCH color space via `palette`
- **Logging**: `tracing` + `tracing-subscriber`

## License

MIT
