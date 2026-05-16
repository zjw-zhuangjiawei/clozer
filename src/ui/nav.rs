//! Navigation items for the main window tabs.

/// Navigation items for the main window tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavItem {
    #[default]
    Words,
    Queue,
    Tags,
    Practice,
    Settings,
}

impl NavItem {
    /// Returns all navigation items in display order.
    pub fn all() -> &'static [NavItem; 5] {
        &[
            NavItem::Words,
            NavItem::Queue,
            NavItem::Tags,
            NavItem::Practice,
            NavItem::Settings,
        ]
    }

    /// Primary navigation items (main application views).
    pub fn main() -> &'static [NavItem; 4] {
        &[
            NavItem::Words,
            NavItem::Queue,
            NavItem::Tags,
            NavItem::Practice,
        ]
    }

    /// Secondary navigation items (utility/views).
    pub fn secondary() -> &'static [NavItem; 1] {
        &[NavItem::Settings]
    }

    /// Returns the i18n key for this navigation item's label.
    pub fn label_key(&self) -> &'static str {
        match self {
            NavItem::Words => "nav-words",
            NavItem::Queue => "nav-queue",
            NavItem::Tags => "nav-tags",
            NavItem::Practice => "nav-practice",
            NavItem::Settings => "nav-settings",
        }
    }

    /// Returns the keyboard shortcut hint for this navigation item.
    pub fn shortcut_key(&self) -> &'static str {
        match self {
            NavItem::Words => "sidebar-shortcut-words",
            NavItem::Queue => "sidebar-shortcut-queue",
            NavItem::Tags => "sidebar-shortcut-tags",
            NavItem::Practice => "sidebar-shortcut-practice",
            NavItem::Settings => "sidebar-shortcut-settings",
        }
    }
}
