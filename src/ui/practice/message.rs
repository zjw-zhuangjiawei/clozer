use crate::models::types::TagId;

#[derive(Debug, Clone)]
pub enum PracticeMessage {
    ToggleTagPicker,
    TagSearchChanged(String),
    TagFilterSelected(TagId),
    TagFilterCleared,
    StartSession,

    EndSession,

    NextCloze,
    PreviousCloze,

    AnswerChanged {
        blank_index: usize,
        value: String,
    },

    SubmitAnswers,
    SkipCloze,

    Notify {
        level: NotificationLevel,
        message: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    Error,
    Warning,
    Info,
}
