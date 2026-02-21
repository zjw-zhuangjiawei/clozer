//! Navigation items for the main window tabs.

/// Navigation items for the main window tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavItem {
    #[default]
    Words,
    Queue,
    Settings,
}

impl NavItem {
    /// Returns the display label for this navigation item.
    pub fn label(&self) -> &'static str {
        match self {
            NavItem::Words => "Words",
            NavItem::Queue => "Queue",
            NavItem::Settings => "Settings",
        }
    }
}
