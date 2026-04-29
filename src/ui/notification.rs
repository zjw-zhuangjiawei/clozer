use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: usize,
    pub level: NotificationLevel,
    pub message: String,
    pub created_at: Instant,
}

impl Notification {
    pub fn new(id: usize, level: NotificationLevel, message: impl Into<String>) -> Self {
        Self {
            id,
            level,
            message: message.into(),
            created_at: Instant::now(),
        }
    }

    pub fn error(id: usize, message: impl Into<String>) -> Self {
        Self::new(id, NotificationLevel::Error, message)
    }

    pub fn warning(id: usize, message: impl Into<String>) -> Self {
        Self::new(id, NotificationLevel::Warning, message)
    }

    pub fn info(id: usize, message: impl Into<String>) -> Self {
        Self::new(id, NotificationLevel::Info, message)
    }
}
