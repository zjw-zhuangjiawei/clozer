# Architecture Modules

**Summary**: Detailed module structure and file organization in the Clozer codebase.

**Why**: Provides a comprehensive map of the codebase structure for navigation and understanding.

---

## Source Structure

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
│   ├── model.rs      # LLM model config
│   ├── provider.rs   # LLM provider config
│   ├── types.rs      # Newtype ID types (WordId, MeaningId, etc.)
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
│   │   └── tags.rs
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
    │   ├── button.rs
    │   ├── checkbox.rs
    │   ├── detail.rs
    │   └── mod.rs
    ├── queue/        # Queue view sub-module
    │   ├── handlers.rs
    │   ├── message.rs
    │   ├── state.rs
    │   ├── update.rs
    │   └── view.rs
    ├── settings/     # Settings view sub-module
    │   ├── handlers.rs
    │   ├── message.rs
    │   ├── state.rs
    │   ├── update.rs
    │   └── view.rs
    └── words/        # Words view sub-module
        ├── detail_view.rs
        ├── handlers.rs
        ├── message.rs
        ├── state.rs
        ├── update.rs
        └── view.rs
```

---

## UI Module Organization

Each UI feature follows a consistent pattern with message, state, update, and view:

```
feature/
├── message.rs    # Message enum for this feature
├── state.rs      # State struct for this feature
├── update.rs    # Update logic
└── view.rs       # View function
```

This pattern aligns with Iced's Elm-like architecture.

---

## Entities

| Entity | Description |
|--------|-------------|
| `Word` | A word to learn, has unique UUID, optional language, and meaning associations |
| `Meaning` | Definition with PartOfSpeech, CefrLevel, and tag associations |
| `Cloze` | Fill-in-the-blank sentence with segments, derived from meaning |
| `Tag` | Hierarchical tag/category with parent-child relationships |
| `Model` | LLM model configuration |
| `Provider` | LLM provider for generating cloze sentences |
| `ProviderType` | Enum for LLM provider types (OpenAI, Anthropic, DeepSeek, Gemini, Ollama, Perplexity, XAI) |
| `QueueItem` | An item in the generation queue |

---

## Related Rules

- [Architecture Layers](./arch-layers.md) - Layer responsibilities
- [Dev Models](./dev-models.md) - Data structures
- [Dev Registry](./dev-registry.md) - In-memory storage
- [Dev Persistence](./dev-persistence.md) - Database patterns
- [Dev UI](./dev-ui.md) - Iced UI patterns
