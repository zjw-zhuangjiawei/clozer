//! Tags panel UI state.
//!
//! Manages tag tree expansion, search, creation, editing, and selection.

use std::collections::HashSet;

use crate::models::types::TagId;
use crate::registry::TagRegistry;

/// State for creating a new tag.
#[derive(Debug, Clone)]
pub struct TagCreationState {
    /// Tag name input
    pub name: String,
    /// Parent tag ID (None for root)
    pub parent_id: Option<TagId>,
}

/// Complete state for Tags panel.
#[derive(Debug, Default)]
pub struct TagsState {
    /// Search query for filtering tags
    pub search: String,
    /// Currently selected tag for detail view
    pub selected: Option<TagId>,
    /// Set of expanded tag IDs in the tree
    pub expanded: HashSet<TagId>,
    /// Active rename operation (tag_id, buffer)
    pub renaming: Option<(TagId, String)>,
    /// Active tag creation state
    pub creation: Option<TagCreationState>,
    /// Tag being reparented
    pub reparenting: Option<TagId>,
    /// Tag pending deletion confirmation
    pub pending_delete: Option<TagId>,
}

impl TagsState {
    /// Creates a new TagsState.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the number of meanings associated with a tag.
    pub fn get_meaning_count(
        &self,
        tag_id: TagId,
        meaning_registry: &crate::registry::MeaningRegistry,
    ) -> usize {
        meaning_registry
            .by_tag
            .get(&tag_id)
            .map(|ids| ids.len())
            .unwrap_or(0)
    }

    /// Check if a tag matches the current search query.
    pub fn matches_search(&self, tag_name: &str) -> bool {
        if self.search.is_empty() {
            return true;
        }
        tag_name
            .to_lowercase()
            .contains(&self.search.to_lowercase())
    }

    /// Collect all tag IDs that match the search (including descendants for context).
    pub fn collect_matching_ids(&self, registry: &TagRegistry) -> HashSet<TagId> {
        let mut matched = HashSet::new();
        for (id, tag) in registry.iter() {
            if self.matches_search(&tag.name) {
                matched.insert(*id);
                // Also include all descendants so tree renders fully
                Self::collect_descendants(*id, registry, &mut matched);
            }
        }
        matched
    }

    fn collect_descendants(tag_id: TagId, registry: &TagRegistry, out: &mut HashSet<TagId>) {
        if let Some(tag) = registry.get(tag_id) {
            for child_id in &tag.children_ids {
                out.insert(*child_id);
                Self::collect_descendants(*child_id, registry, out);
            }
        }
    }

    /// Expand all tags that have children.
    pub fn expand_all(&mut self, registry: &TagRegistry) {
        self.expanded.clear();
        for (id, tag) in registry.iter() {
            if !tag.children_ids.is_empty() {
                self.expanded.insert(*id);
            }
        }
    }

    /// Collapse all tags.
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }
}
