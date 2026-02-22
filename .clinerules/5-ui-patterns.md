# UI Patterns (Iced)

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
└── words/       # Words view sub-module
    ├── message.rs
    ├── state.rs
    ├── update.rs
    ├── view.rs
    └── detail_view.rs
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

## Selection State Pattern

Selection is embedded in UI state. Meanings are tracked directly; word selection is derived. Cloze selection is independent.

```rust
// WordsUiState uses HashSet for O(1) operations
#[derive(Debug, Default)]
pub struct WordsUiState {
    // Selection
    // Meanings (words are derived from meanings)
    pub selected_meaning_ids: HashSet<Uuid>,
    // Clozes (independent selection)
    pub selected_cloze_ids: HashSet<Uuid>,
}

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
            self.selected_meaning_ids.extend(word.meaning_ids.iter());
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

    /// Toggle a cloze's selection.
    pub fn toggle_cloze_selection(&mut self, cloze_id: Uuid) {
        if self.selected_cloze_ids.contains(&cloze_id) {
            self.selected_cloze_ids.remove(&cloze_id);
        } else {
            self.selected_cloze_ids.insert(cloze_id);
        }
    }

    /// Clear all selections.
    pub fn clear_selection(&mut self) {
        self.selected_meaning_ids.clear();
        self.selected_cloze_ids.clear();
    }

    /// Get total selection count (meanings + clozes).
    pub fn total_selection_count(&self) -> usize {
        self.selected_meaning_ids.len() + self.selected_cloze_ids.len()
    }
}
```

---

## Cloze Filter Pattern

Filter meanings by cloze generation status:

```rust
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

## Detail Panel Selection

Track what is currently shown in the detail panel:

```rust
/// Selection for the details panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailSelection {
    Word(Uuid),
    Meaning(Uuid),
    Cloze(Uuid),
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

- [Architecture](./1-architecture.md) - Module structure
- [Models](./2-models.md) - Data structures
- [Logging](./7-logging.md) - Tracing patterns
