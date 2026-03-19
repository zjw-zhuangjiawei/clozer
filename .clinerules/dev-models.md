# Development: Models

**Summary**: Pure data structures with typed_builder for complex construction.

**Why**: Models define the core data entities. Using typed_builder ensures valid construction and provides a fluent API.

---

## Word Model

Represents a word to learn with optional language and meaning associations.

```rust
// From src/models/word.rs
use langtag::LangTagBuf;
use std::collections::BTreeSet;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_meaning(&mut self, meaning_id: Uuid) {
        self.meaning_ids.insert(meaning_id);
    }
))]
pub struct Word {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub content: String,
    #[builder(default, via_mutators)]
    pub meaning_ids: BTreeSet<Uuid>,
    #[builder(default, setter(strip_option))]
    pub language: Option<LangTagBuf>,
}
```

### Usage

```rust
// Create a word with a meaning
let word = Word::builder()
    .content("hello")
    .with_meaning(meaning_id)
    .build();

// Create a word with language
let word = Word::builder()
    .content("bonjour")
    .language(Some("fr".into()))
    .build();
```

---

## Meaning Model

Represents a definition of a word with part of speech, CEFR level, and tags.

```rust
// From src/models/meaning.rs
use std::collections::BTreeSet;
use strum::{Display, VariantArray};
use typed_builder::TypedBuilder;
use uuid::Uuid;

/// Part of speech categories for classifying words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum PartOfSpeech {
    // Major
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    // Articles & Modals
    Article,
    Modal,
    // Other
    Numeral,
    Abbreviation,
}

/// CEFR (Common European Framework of Reference) language proficiency levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum CefrLevel {
    /// A1: Beginner
    A1,
    /// A2: Elementary
    A2,
    /// B1: Intermediate
    B1,
    /// B2: Upper Intermediate
    B2,
    /// C1: Advanced
    C1,
    /// C2: Proficient
    C2,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_tag(&mut self, tag_id: Uuid) {
        self.tag_ids.insert(tag_id);
    }
))]
pub struct Meaning {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub word_id: Uuid,
    pub definition: String,
    pub pos: PartOfSpeech,
    #[builder(default)]
    pub cefr_level: Option<CefrLevel>,
    #[builder(default, via_mutators)]
    pub tag_ids: BTreeSet<Uuid>,
}
```

### Usage

```rust
// Create a meaning with tags
let meaning = Meaning::builder()
    .word_id(word_id)
    .definition("A greeting")
    .pos(PartOfSpeech::Noun)
    .cefr_level(Some(CefrLevel::A1))
    .with_tag(tag_id)
    .build();
```

---

## Cloze Model

Represents a fill-in-the-blank sentence with segments.

```rust
// From src/models/cloze.rs
use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::fmt;
use typed_builder::TypedBuilder;
use uuid::Uuid;

static BLANK_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());

/// A segment of a cloze sentence - either plain text or a blank with answer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClozeSegment {
    Text(String),
    Blank(String),
}

impl fmt::Display for ClozeSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClozeSegment::Text(s) => write!(f, "{}", s),
            ClozeSegment::Blank(a) => write!(f, "[{}]", a),
        }
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Cloze {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub meaning_id: Uuid,
    pub segments: Vec<ClozeSegment>,
}
```

### Cloze Methods

```rust
impl Cloze {
    /// Parse a sentence with `[answer]` markers into segments
    pub fn parse_from_sentence(sentence: &str) -> Vec<ClozeSegment> { ... }

    /// Render sentence with blanks visible as `___`
    pub fn render_blanks(&self) -> String { ... }

    /// Render sentence with answers filled in
    pub fn render_answers(&self) -> String { ... }
}
```

### Usage

```rust
// Parse a sentence into cloze segments
let segments = Cloze::parse_from_sentence("The [cat] sat on the [mat]");
assert_eq!(segments.len(), 5); // Text, Blank, Text, Blank, Text

// Render as blanks
let cloze = Cloze::builder()
    .meaning_id(meaning_id)
    .segments(segments)
    .build();
let blanks = cloze.render_blanks(); // "The ___ sat on the ___"
let answers = cloze.render_answers(); // "The cat sat on the mat"
```

---

## Tag Model

Represents a hierarchical tag/category for organizing meanings.

```rust
// From src/models/tag.rs
use std::collections::BTreeSet;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_child(&mut self, child_id: Uuid) {
        self.children_ids.insert(child_id);
    }
))]
pub struct Tag {
    #[builder(default=Uuid::new_v4())]
    pub id: Uuid,
    pub name: String,
    #[builder(default)]
    pub parent_id: Option<Uuid>,
    #[builder(default, via_mutators)]
    pub children_ids: BTreeSet<Uuid>,
}
```

### Usage

```rust
// Create a parent tag
let parent_tag = Tag::builder()
    .name("Colors")
    .build();

// Create a child tag
let child_tag = Tag::builder()
    .name("Red")
    .parent_id(Some(parent_tag.id))
    .with_child(child_id) // Add grandchild
    .build();
```

---

## Derive Conventions

Add these derives to data models:

```rust
#[derive(Debug, Clone)]       // For cloning in state updates
#[derive(PartialEq, Eq)]      // For comparisons
pub struct Word { ... }

#[derive(Debug, Clone)]       // For message passing
pub enum Message { ... }
```

---

## Related Rules

- [Reference: API Design](./ref-api.md) - Builder pattern details
- [Dev: Registry](./dev-registry.md) - In-memory storage
- [Dev: Persistence](./dev-persistence.md) - DTO serialization
- [Architecture Layers](./arch-layers.md) - Layer overview
