# Development: UI Patterns

**Summary**: Iced GUI patterns following Elm architecture - Model/View/Update pattern with message routing.

**Why**: Provides consistent patterns for building the desktop UI with proper state management and event handling.

---

## Elm-like Architecture

Iced follows the Elm architecture:

```
Model → View (render UI)
Message → Update (handle events)
```

```rust
// app.rs
pub struct App {
    config: AppConfig,
    pub app_state: AppState,
    pub window_state: MainWindowState,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Main(msg) => ui::app::update(
                &mut self.window_state,
                msg,
                &mut self.app_state.model,
                iced::window::Id::unique(),
            ),
            // ...
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        ui::app::view(&self.window_state, &self.app_state.model).map(Message::Main)
    }
}
```

---

## UI Module Structure

The UI is organized with flat sub-modules under `src/ui/`:

```
src/ui/
├── mod.rs
├── app.rs        # App-level view/update functions
├── message.rs    # MainWindowMessage enum
├── state.rs      # MainWindowState
├── nav.rs        # Navigation items
├── theme.rs      # Theme definitions (AppTheme, ThemeColors)
├── components/
│   ├── button.rs    # Reusable button component
│   ├── checkbox.rs
│   ├── detail.rs    # Detail panel component
│   ├── dsl/         # Declarative DSL components
│   │   ├── badge.rs
│   │   ├── button.rs
│   │   ├── card.rs
│   │   ├── input.rs
│   │   ├── row.rs
│   │   └── mod.rs
│   └── mod.rs
├── layout/          # Adaptive layout system
│   ├── adaptive.rs
│   ├── breakpoint.rs
│   ├── grid.rs
│   ├── mode.rs
│   ├── waterfall.rs
│   └── mod.rs
├── queue/        # Queue view sub-module
│   ├── handlers.rs  # Event handlers
│   ├── message.rs
│   ├── state.rs
│   ├── update.rs
│   └── view.rs
├── settings/     # Settings view sub-module
│   ├── handlers.rs  # Event handlers
│   ├── message.rs
│   ├── state.rs
│   ├── update.rs
│   └── view.rs
└── words/       # Words view sub-module
    ├── manager/     # State management modules
    │   ├── detail.rs
    │   ├── edit.rs
    │   ├── expansion.rs
    │   ├── search.rs
    │   ├── selection.rs
    │   └── mod.rs
    ├── detail_view.rs
    ├── handlers.rs  # Event handlers
    ├── message.rs
    ├── state.rs
    ├── update.rs
    └── view.rs
```

Each UI feature follows a consistent pattern:

```
feature/
├── handlers.rs   # Event handlers (button clicks, input changes, etc.)
├── message.rs    # Message enum for this feature
├── state.rs      # State struct for this feature
├── update.rs     # Update logic
└── view.rs       # View function
```

---

## Message Naming

Messages follow a hierarchical routing pattern (single-window):

```rust
// Top-level Message (src/message.rs)
#[derive(Debug, Clone)]
pub enum Message {
    // Route to main window (single window, no ID needed)
    Main(MainWindowMessage),

    // Global messages
    QueueGenerationResult(QueueGenerationResult),

    // Application close request (from subscription)
    CloseRequested,
}

// Main window messages (src/ui/message.rs)
#[derive(Debug, Clone)]
pub enum MainWindowMessage {
    Words(WordsMessage),
    Queue(QueueMessage),
    Settings(SettingsMessage),
    Navigate(NavItem),
}
```

---

## WordsState (Manager Pattern)

The Words panel uses a Manager pattern for organized state management:

```rust
// From src/ui/words/state.rs
/// Complete state for Words panel using Manager pattern.
#[derive(Debug, Default)]
pub struct WordsState {
    /// Search and filter manager
    pub search: SearchManager,
    /// Selection manager
    pub selection: SelectionManager,
    /// Expansion manager
    pub expansion: ExpansionManager,
    /// Detail panel manager
    pub detail: DetailManager,
    /// Edit session manager
    pub edit: EditManager,
}
```

### Manager Modules

Each manager handles a specific aspect of UI state:

```rust
// From src/ui/words/manager/mod.rs
pub mod detail;      // DetailManager, DetailSelection, TagDropdownState
pub mod edit;        // EditManager, EditBuffer, EditContext
pub mod expansion;   // ExpansionManager
pub mod search;      // SearchManager
pub mod selection;   // SelectionManager
```

**SearchManager**: Search query and filter state
**SelectionManager**: Selection state for meanings and clozes
**ExpansionManager**: Word expansion state (which words are expanded)
**DetailManager**: Detail panel selection and tag dropdown state
**EditManager**: Edit session context and buffers

---

## Filter State

Filter words by cloze status and tags:

```rust
/// Filter state for the words tree.
#[derive(Debug, Clone, Default)]
pub struct FilterState {
    pub cloze_status: ClozeFilter,
    pub tag_id: Option<Uuid>,
}

/// Filter state for cloze generation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display)]
pub enum ClozeFilter {
    #[default]
    All,
    HasClozes,
    Pending,
    Failed,
}
```

---

## Detail Selection

Track what is currently shown in the detail panel:

```rust
/// Selection for the details panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailSelection {
    Word(Uuid),
    Meaning(Uuid),
    Cloze(Uuid),
}

/// What is currently being edited in the detail panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetailEditMode {
    /// Not editing anything
    #[default]
    None,
    /// Editing a word
    Word(Uuid),
    /// Editing a meaning
    Meaning(Uuid),
}

/// Buffer for storing edits in progress.
#[derive(Debug, Clone)]
pub struct EditBuffer {
    pub word_content: String,
    pub meaning_definition: String,
    pub meaning_pos: PartOfSpeech,
    pub meaning_cefr: Option<CefrLevel>,
}
```

---

## Meaning Input State

Input state for creating meanings:

```rust
/// Input state for creating meanings.
#[derive(Debug, Clone)]
pub struct MeaningInputState {
    pub definition: String,
    pub pos: PartOfSpeech,
    pub cefr_level: Option<CefrLevel>,
}

impl Default for MeaningInputState {
    fn default() -> Self {
        Self {
            definition: String::new(),
            pos: PartOfSpeech::Noun,
            cefr_level: None,
        }
    }
}
```

---

## Selection State Pattern

Selection is embedded in UI state. Meanings are tracked directly; word selection is derived.

```rust
impl WordsUiState {
    /// Check if a word is "fully selected" (all its meanings are selected).
    pub fn is_word_selected(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        word.meaning_ids
            .iter()
            .all(|mid| self.selected_meaning_ids.contains(mid))
    }

    /// Check if a word is "partially selected" (some but not all meanings selected).
    pub fn is_word_partial(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        let selected_count = word
            .meaning_ids
            .iter()
            .filter(|mid| self.selected_meaning_ids.contains(*mid))
            .count();
        selected_count > 0 && selected_count < word.meaning_ids.len()
    }

    /// Toggle word selection (select all meanings or deselect all).
    pub fn toggle_word_selection(&mut self, word: &Word) {
        if self.is_word_selected(word) {
            for mid in &word.meaning_ids {
                self.selected_meaning_ids.remove(mid);
            }
        } else {
            for mid in &word.meaning_ids {
                self.selected_meaning_ids.insert(*mid);
            }
        }
    }

    /// Toggle a single meaning's selection.
    pub fn toggle_meaning_selection(&mut self, meaning_id: Uuid) {
        if self.selected_meaning_ids.contains(&meaning_id) {
            self.selected_meaning_ids.remove(&meaning_id);
        } else {
            self.selected_meaning_ids.insert(meaning_id);
        }
    }

    /// Get the count of selected meanings.
    pub fn selected_count(&self) -> usize {
        self.selected_meaning_ids.len()
    }

    /// Check if there are any selected meanings.
    pub fn has_selection(&self) -> bool {
        !self.selected_meaning_ids.is_empty()
    }

    /// Clear all selections.
    pub fn clear_selection(&mut self) {
        self.selected_meaning_ids.clear();
        self.selected_cloze_ids.clear();
    }
}
```

---

## Dropdown State Pattern

Tag dropdown uses a struct-based approach with target enum:

```rust
/// Target for tag dropdown operations.
#[derive(Debug, Clone)]
pub enum TagDropdownTarget {
    /// Batch operation on selected meanings
    SelectedMeanings,
    /// Single meaning operation
    SingleMeaning(Uuid),
}

/// State for the tag dropdown.
#[derive(Debug, Clone)]
pub struct TagDropdownState {
    pub target: TagDropdownTarget,
    pub search: String,
}

impl TagDropdownState {
    pub fn new(target: TagDropdownTarget) -> Self {
        Self {
            target,
            search: String::new(),
        }
    }
}
```

---

## QueueState

Queue panel state with selection management:

```rust
// From src/ui/queue/state.rs
use std::collections::HashSet;

/// Selection state for queue items.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Selected queue item IDs
    pub items: HashSet<Uuid>,
}

impl SelectionState {
    pub fn toggle(&mut self, item_id: Uuid) { ... }
    pub fn is_selected(&self, item_id: Uuid) -> bool { ... }
    pub fn select_all(&mut self, item_ids: impl IntoIterator<Item = Uuid>) { ... }
    pub fn clear(&mut self) { ... }
    pub fn count(&self) -> usize { ... }
}

/// Queue panel state.
#[derive(Debug, Default)]
pub struct QueueState {
    pub selection: SelectionState,
}
```

---

## SettingsState

Settings panel state with provider and model editing:

```rust
// From src/ui/settings/state.rs
/// Editing state for providers.
#[derive(Debug, Clone)]
pub struct ProviderEditState {
    pub editing_id: Option<Uuid>,
    pub data: ProviderConfig,
    pub is_new: bool,
}

/// Editing state for models.
#[derive(Debug, Clone)]
pub struct ModelEditState {
    pub editing_id: Option<Uuid>,
    pub data: ModelConfig,
    pub is_new: bool,
}

/// Settings panel state.
#[derive(Debug, Default)]
pub struct SettingsState {
    pub provider_edit: ProviderEditState,
    pub model_edit: ModelEditState,
}
```

---

## Layout System

Adaptive layout system supporting multiple modes:

```rust
// From src/ui/layout/mod.rs
pub enum LayoutMode {
    Adaptive,   // Single column or master-detail based on breakpoint
    Grid,       // Multi-column evenly distributed
    Waterfall,  // Staggered arrangement for varying heights
}

pub struct LayoutConfig {
    pub mode: LayoutMode,
    pub columns: usize,
}

pub fn build_layout<'a, M: 'a>(
    config: &LayoutConfig,
    nav_bar: Element<'a, M>,
    content: Element<'a, M>,
    breakpoint: ThemeBreakpoint,
) -> Element<'a, M> { ... }
```

### Breakpoint System

```rust
// From src/ui/layout/breakpoint.rs
pub enum Breakpoint {
    Mobile,   // < 768px
    Tablet,   // 768px - 1024px
    Desktop,  // 1024px - 1440px
    Wide,     // >= 1440px
}
```

---

## DSL Components

Declarative DSL component library for consistent UI construction:

```rust
// From src/ui/components/dsl/mod.rs
pub use badge::{Badge, BadgeStyle, badge, cefr_badge, pos_badge};
pub use button::{
    ButtonBuilder, ButtonStyle, ButtonVariant, button, danger_btn, primary_btn, secondary_btn,
};
pub use card::{Card, CardStyle, card};
pub use row::{RowBuilder, h_stack, row, v_stack};
```

### Badge Components

```rust
// Part of speech badge
let pos = pos_badge(PartOfSpeech::Noun);

// CEFR level badge
let cefr = cefr_badge(CefrLevel::A1);
```

### Button Components

```rust
// Builder pattern for custom buttons
let my_btn = button::<Message>("Click me")
    .style(ButtonStyle::Primary)
    .on_press(Message::Submit);

// Predefined button variants
let primary = primary_btn("Save");
let secondary = secondary_btn("Cancel");
let danger = danger_btn("Delete");
```

### Card Components

```rust
// Fluent card builder
let my_card = card::<Message>()
    .padding(16.0)
    .push(some_element)
    .build();
```

### Row/Stack Components

```rust
// Horizontal stack
let h = h_stack(vec![element1, element2]);

// Vertical stack
let v = v_stack(vec![element1, element2]);

// Builder pattern
let row = row::<Message>()
    .spacing(8)
    .push(element1)
    .push(element2)
    .build();
```

---

## Window Close Handling

Handle window close events to save config before exit:

```rust
// Message variant
WindowCloseRequested(iced::window::Id),

// Handler in update
Message::WindowCloseRequested(window_id) => {
    // Save config before closing
    if let Some(config) = self.config.as_ref() {
        if let Err(e) = config.save_to_file() {
            tracing::error!("Failed to save config: {}", e);
        }
    }
    // Exit the application
    self.should_exit = true;
}
```

---

## Component Message Mapping

Use `.map()` to transform child messages to parent messages.

```rust
// Parent component (ui/app.rs)
pub fn view(&self) -> Element<'_, Message> {
    let child_view = ui::words::view(
        &self.state.words,
        // ...
    )
    .map(Message::Words)  // Transform child → parent
    .into()
}

// Child component (ui/words/view.rs)
#[derive(Debug, Clone)]
pub enum WordsMessage {
    InputChanged(String),
    ToggleTagDropdown,
}

pub fn view(...) -> Element<'_, WordsMessage> {
    // ...
}
```

---

## View Function Signature

Pass all needed state as parameters:

```rust
pub fn words_view(
    word_registry: &WordRegistry,
    meaning_registry: &MeaningRegistry,
    cloze_registry: &ClozeRegistry,
    tag_registry: &TagRegistry,
    state: &WordsUiState,
) -> Element<'_, Message> {
    // Build UI
}
```

---

## Task Integration

Return `Task<Message>` from `update()` for async operations:

```rust
pub fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::QueueProcess => {
            let generator = self.generator.generator();
            return self.queue.process(
                &generator,
                &self.data.word_registry,
                &self.data.meaning_registry,
            );
        }
        _ => {}
    }
    Task::none()
}
```

---

## Related Rules

- [Architecture Layers](./arch-layers.md) - Layer responsibilities
- [Architecture Modules](./arch-modules.md) - Module structure
- [Dev Models](./dev-models.md) - Data structures
- [Dev Logging](./dev-logging.md) - Tracing patterns
- [Dev Error Handling](./dev-error-handling.md) - Error handling patterns
- [Dev Testing](./dev-testing.md) - Testing patterns
