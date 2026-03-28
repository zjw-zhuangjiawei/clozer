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

## WordsUiState

The main UI state struct for the words panel:

```rust
// From src/ui/words/state.rs
#[derive(Debug, Default)]
pub struct WordsUiState {
    // Search & Filter
    pub search_query: String,
    pub filter: FilterState,

    // Expansion
    pub expanded_word_ids: HashSet<Uuid>,

    // Add meaning
    pub adding_meaning_to_word: Option<Uuid>,
    pub meaning_input: MeaningInputState,

    // Selection
    pub selected_meaning_ids: HashSet<Uuid>,
    pub selected_cloze_ids: HashSet<Uuid>,

    // Tag dropdown
    pub tag_dropdown: Option<TagDropdownState>,

    // Detail panel selection
    pub selected_detail: Option<DetailSelection>,

    // Detail panel editing
    pub detail_edit_mode: DetailEditMode,
    pub edit_buffer: EditBuffer,
}
```

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
