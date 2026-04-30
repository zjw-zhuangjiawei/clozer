//! Tags panel message types.
//!
//! Messages are flattened for direct state manipulation.

use crate::models::types::TagId;

/// Flattened message enum for Tags panel.
#[derive(Debug, Clone)]
pub enum TagsMessage {
    /// Search query changed
    SearchQueryChanged(String),
    /// Clear search query
    SearchCleared,

    /// Expand a tag in the tree
    TagExpanded(TagId),
    /// Collapse a tag in the tree
    TagCollapsed(TagId),
    /// Expand all tags
    ExpandAll,
    /// Collapse all tags
    CollapseAll,

    /// Select a tag for detail view
    TagSelected(TagId),
    /// Close detail panel
    DetailClosed,

    /// Start creating a new tag
    NewTagStarted,
    /// New tag name changed
    NewTagNameChanged(String),
    /// New tag parent changed
    NewTagParentChanged(Option<TagId>),
    /// Save new tag
    NewTagSaved,
    /// Cancel new tag creation
    NewTagCancelled,

    /// Start renaming a tag
    RenameStarted(TagId),
    /// Rename input changed
    RenameChanged(String),
    /// Save rename
    RenameSaved(TagId),
    /// Cancel rename
    RenameCancelled,

    /// Request tag deletion (show confirmation)
    DeleteRequested(TagId),
    /// Confirm tag deletion
    DeleteConfirmed(TagId),
    /// Cancel deletion
    DeleteCancelled,

    /// Start reparenting a tag
    ReparentStarted(TagId),
    /// Reparent target changed
    ReparentChanged(Option<TagId>),
    /// Save reparent
    ReparentSaved(TagId),
    /// Cancel reparent
    ReparentCancelled,

    /// Navigate to Words panel filtered by this tag
    NavigateToMeanings(TagId),
}
