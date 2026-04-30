//! Tags panel update handler.

use crate::models::Tag;
use crate::models::types::TagId;
use crate::registry::TagRegistry;
use crate::state::Model;
use crate::ui::tags::message::TagsMessage;
use crate::ui::tags::state::{TagCreationState, TagsState};
use iced::Task;

/// Handle tag messages and update state/model.
pub fn update(state: &mut TagsState, message: TagsMessage, model: &mut Model) -> Task<TagsMessage> {
    match message {
        // Search
        TagsMessage::SearchQueryChanged(query) => {
            state.search = query;
        }
        TagsMessage::SearchCleared => {
            state.search.clear();
        }

        // Expand / Collapse
        TagsMessage::TagExpanded(tag_id) => {
            state.expanded.insert(tag_id);
        }
        TagsMessage::TagCollapsed(tag_id) => {
            state.expanded.remove(&tag_id);
        }
        TagsMessage::ExpandAll => {
            state.expand_all(&model.tag_registry);
        }
        TagsMessage::CollapseAll => {
            state.collapse_all();
        }

        // Selection
        TagsMessage::TagSelected(tag_id) => {
            state.selected = Some(tag_id);
            state.creation = None;
            state.renaming = None;
            state.reparenting = None;
            state.pending_delete = None;
        }
        TagsMessage::DetailClosed => {
            state.selected = None;
            state.creation = None;
            state.renaming = None;
            state.reparenting = None;
            state.pending_delete = None;
        }

        // Create
        TagsMessage::NewTagStarted => {
            state.creation = Some(TagCreationState {
                name: String::new(),
                parent_id: state.selected,
            });
            state.selected = None;
            state.renaming = None;
            state.reparenting = None;
            state.pending_delete = None;
        }
        TagsMessage::NewTagNameChanged(name) => {
            if let Some(ref mut creation) = state.creation {
                creation.name = name;
            }
        }
        TagsMessage::NewTagParentChanged(parent_id) => {
            if let Some(ref mut creation) = state.creation {
                creation.parent_id = parent_id;
            }
        }
        TagsMessage::NewTagSaved => {
            if let Some(ref creation) = state.creation {
                let trimmed = creation.name.trim();
                if !trimmed.is_empty()
                    && !model
                        .tag_registry
                        .iter()
                        .any(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase())
                {
                    let mut tag = Tag::builder().name(trimmed.to_string()).build();
                    tag.parent_id = creation.parent_id;

                    let tag_id = tag.id;

                    if let Some(parent_id) = creation.parent_id
                        && let Some(parent) = model.tag_registry.get_mut(parent_id)
                    {
                        parent.children_ids.insert(tag_id);
                    }

                    model.tag_registry.add(tag);
                    tracing::debug!(tag_id = %tag_id, name = %trimmed, "Created tag");
                    state.selected = Some(tag_id);
                }
            }
            state.creation = None;
        }
        TagsMessage::NewTagCancelled => {
            state.creation = None;
        }

        // Rename
        TagsMessage::RenameStarted(tag_id) => {
            if let Some(tag) = model.tag_registry.get(tag_id) {
                state.renaming = Some((tag_id, tag.name.clone()));
                state.creation = None;
                state.reparenting = None;
                state.pending_delete = None;
            }
        }
        TagsMessage::RenameChanged(name) => {
            if let Some((_, ref mut buffer)) = state.renaming {
                *buffer = name;
            }
        }
        TagsMessage::RenameSaved(tag_id) => {
            if let Some((_, new_name)) = state.renaming.take() {
                let trimmed = new_name.trim();
                if !trimmed.is_empty() {
                    let duplicate = model.tag_registry.iter().any(|(id, t)| {
                        *id != tag_id && t.name.to_lowercase() == trimmed.to_lowercase()
                    });
                    if !duplicate && let Some(tag) = model.tag_registry.get_mut(tag_id) {
                        tag.name = trimmed.to_string();
                        tracing::debug!(tag_id = %tag_id, name = %trimmed, "Renamed tag");
                    }
                }
            }
        }
        TagsMessage::RenameCancelled => {
            state.renaming = None;
        }

        // Delete
        TagsMessage::DeleteRequested(tag_id) => {
            state.pending_delete = Some(tag_id);
            state.creation = None;
            state.renaming = None;
            state.reparenting = None;
        }
        TagsMessage::DeleteConfirmed(tag_id) => {
            delete_tag_recursively(tag_id, &mut model.tag_registry, &mut model.meaning_registry);
            if state.selected == Some(tag_id) {
                state.selected = None;
            }
            state.expanded.remove(&tag_id);
            state.pending_delete = None;
            tracing::info!(tag_id = %tag_id, "Deleted tag and descendants");
        }
        TagsMessage::DeleteCancelled => {
            state.pending_delete = None;
        }

        // Reparent
        TagsMessage::ReparentStarted(tag_id) => {
            state.reparenting = Some(tag_id);
            state.creation = None;
            state.renaming = None;
            state.pending_delete = None;
        }
        TagsMessage::ReparentChanged(parent_id) => {
            if let Some(current_id) = state.reparenting {
                // Prevent self-parenting or circular parenting
                if let Some(target_id) = parent_id
                    && (target_id == current_id
                        || is_descendant(target_id, current_id, &model.tag_registry))
                {
                    return Task::none();
                }

                // Read old parent ID first
                let old_parent_id = model.tag_registry.get(current_id).and_then(|t| t.parent_id);

                // Remove from old parent
                if let Some(old_parent_id) = old_parent_id
                    && let Some(old_parent) = model.tag_registry.get_mut(old_parent_id)
                {
                    old_parent.children_ids.remove(&current_id);
                }

                // Update current tag's parent
                if let Some(tag) = model.tag_registry.get_mut(current_id) {
                    tag.parent_id = parent_id;
                }

                // Add to new parent
                if let Some(new_parent_id) = parent_id
                    && let Some(new_parent) = model.tag_registry.get_mut(new_parent_id)
                {
                    new_parent.children_ids.insert(current_id);
                }

                tracing::debug!(
                    tag_id = %current_id,
                    new_parent = ?parent_id,
                    "Reparented tag"
                );
                state.reparenting = None;
            }
        }
        TagsMessage::ReparentSaved(_) => {
            state.reparenting = None;
        }
        TagsMessage::ReparentCancelled => {
            state.reparenting = None;
        }

        // Navigation handled at compositor level
        TagsMessage::NavigateToMeanings(_) => {}
    }

    Task::none()
}

/// Recursively delete a tag and all its descendants.
/// Removes tag references from all meanings.
fn delete_tag_recursively(
    tag_id: TagId,
    tag_registry: &mut TagRegistry,
    meaning_registry: &mut crate::registry::MeaningRegistry,
) {
    let children: Vec<TagId> = if let Some(tag) = tag_registry.get(tag_id) {
        tag.children_ids.iter().copied().collect()
    } else {
        return;
    };

    // Recursively delete children first
    for child_id in children {
        delete_tag_recursively(child_id, tag_registry, meaning_registry);
    }

    // Remove this tag from all meanings
    let meaning_ids: Vec<crate::models::types::MeaningId> = meaning_registry
        .iter_by_tag(tag_id)
        .map(|(id, _)| *id)
        .collect();
    for meaning_id in meaning_ids {
        meaning_registry.remove_tag(meaning_id, tag_id);
    }

    // Remove from parent
    let parent_id = tag_registry.get(tag_id).and_then(|t| t.parent_id);
    if let Some(parent_id) = parent_id
        && let Some(parent) = tag_registry.get_mut(parent_id)
    {
        parent.children_ids.remove(&tag_id);
    }

    tag_registry.delete(tag_id);
}

/// Check if `candidate` is a descendant of `ancestor`.
fn is_descendant(ancestor: TagId, candidate: TagId, registry: &TagRegistry) -> bool {
    if let Some(tag) = registry.get(candidate)
        && let Some(parent_id) = tag.parent_id
    {
        if parent_id == ancestor {
            return true;
        }
        return is_descendant(ancestor, parent_id, registry);
    }
    false
}
