//! Application UI state.

use crate::ui::AppTheme;
use crate::ui::nav::NavItem;
use crate::ui::notification::{Notification, NotificationLevel};
use crate::ui::settings::state::SettingsState;
use crate::ui::words::state::WordsState;

/// UI presentation state for the single-window application.
///
/// Contains sub-panel states, navigation, window dimensions, and theme.
/// Queue panel has no local state — selection is managed in QueueRegistry.
#[derive(Debug)]
pub struct UiState {
    /// Words panel state
    pub words: WordsState,
    /// Settings panel state
    pub settings: SettingsState,
    /// Current navigation view
    pub current_view: NavItem,
    /// Current window width for responsive layout
    pub window_width: u16,
    /// Current UI theme
    pub theme: AppTheme,
    /// Active notifications displayed to the user
    pub notifications: Vec<Notification>,
    /// Next notification ID (monotonically increasing)
    pub next_notification_id: usize,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            words: WordsState::new(),
            settings: SettingsState::new(),
            current_view: NavItem::default(),
            window_width: 1024,
            theme: AppTheme::Light,
            notifications: Vec::new(),
            next_notification_id: 0,
        }
    }
}

impl UiState {
    /// Creates a new UiState.
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a notification and assign it a unique ID.
    pub fn push_notification(
        &mut self,
        level: NotificationLevel,
        message: impl Into<String>,
    ) -> usize {
        let id = self.next_notification_id;
        self.next_notification_id += 1;
        self.notifications
            .push(Notification::new(id, level, message));
        id
    }

    /// Remove a notification by ID.
    pub fn dismiss_notification(&mut self, id: usize) {
        self.notifications.retain(|n| n.id != id);
    }

    /// Remove expired notifications (older than 5 seconds).
    pub fn clean_expired(&mut self) {
        self.notifications
            .retain(|n| n.created_at.elapsed().as_secs() < 5);
    }
}
