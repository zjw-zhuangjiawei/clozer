# Reference: API Design

**Summary**: API patterns including builder, strum enums, From/Into traits, and trait implementations.

**Why**: Provides consistent patterns for creating types and implementing conversions across the codebase.

---

## Builder Pattern with typed_builder

Use typed_builder for complex construction with optional fields.

```rust
use std::collections::BTreeSet;

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

// Usage
let word = Word::builder()
    .content("example")
    .with_meaning(meaning_id)
    .build();

// Meaning with tag mutator
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

// Hierarchical tag with children
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

---

## Enum Derives with strum

Use strum for enum derives like Display and VariantArray.

```rust
use strum::{Display, VariantArray};

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

// Usage: Get all variants, convert to string
let variants = PartOfSpeech::VARIANTS;
let label = PartOfSpeech::Noun.to_string();
```

---

## Flexible APIs with `impl Into<T>`

Accept `impl Into<T>` to reduce `.into()` calls at call sites.

```rust
// Good: Flexible API
fn create_word(content: impl Into<String>) -> Word {
    Word::builder().content(content.into()).build()
}

// Bad: Inflexible
fn create_word(content: String) -> Word {
    Word::builder().content(content).build()
}
```

---

## Implement `From<T>`, Not `Into<T>`

The blanket implementation is provided automatically.

```rust
// Good: Implement From
impl From<String> for Word {
    fn from(content: String) -> Self {
        Word::builder().content(content).build()
    }
}

// Usage
let word: Word = "hello".into();
```

---

## Derive Conventions

Add these derives to data models and messages:

```rust
#[derive(Debug, Clone)]  // For cloning in state updates
#[derive(PartialEq, Eq)] // For comparisons
pub struct Word { ... }

#[derive(Debug, Clone)]  // For message passing
pub enum Message { ... }
```

---

## Related Rules

- [Dev: Models](./dev-models.md) - Data structure patterns
- [Dev: Persistence](./dev-persistence.md) - DTO pattern
- [Dev: Registry](./dev-registry.md) - In-memory storage
