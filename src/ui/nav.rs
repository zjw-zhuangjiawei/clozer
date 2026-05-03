//! Navigation items for the main window tabs.

/// Navigation items for the main window tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavItem {
    #[default]
    Words,
    Queue,
    Tags,
    Settings,
}

impl NavItem {
    /// Returns all navigation items in display order.
    pub fn all() -> &'static [NavItem; 4] {
        &[
            NavItem::Words,
            NavItem::Queue,
            NavItem::Tags,
            NavItem::Settings,
        ]
    }

    /// Primary navigation items (main application views).
    pub fn main() -> &'static [NavItem; 3] {
        &[NavItem::Words, NavItem::Queue, NavItem::Tags]
    }

    /// Secondary navigation items (utility/views).
    pub fn secondary() -> &'static [NavItem; 1] {
        &[NavItem::Settings]
    }

    /// Returns the display label for this navigation item.
    pub fn label(&self) -> &'static str {
        match self {
            NavItem::Words => "Words",
            NavItem::Queue => "Queue",
            NavItem::Tags => "Tags",
            NavItem::Settings => "Settings",
        }
    }
}
