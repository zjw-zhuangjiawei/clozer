//! Tags panel view.
//!
//! Two-column layout: tag tree on the left, detail/form on the right.

use std::collections::HashSet;

use crate::models::Tag;
use crate::models::types::TagId;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::tags::message::TagsMessage;
use crate::ui::tags::state::{TagCreationState, TagsState};
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::AdvancedInput;
use crate::ui::widgets::button;
use crate::ui::widgets::container::card;
use crate::ui::widgets::text as txt;
use iced::Element;
use iced::widget::{Button, Column, Container, PickList, Row, rule, scrollable, text};

/// PickList adapter for parent tag selection.
#[derive(Debug, Clone, PartialEq)]
struct ParentOption {
    id: Option<TagId>,
    label: String,
}

impl std::fmt::Display for ParentOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

/// Main tags panel view.
pub fn view<'a>(state: &'a TagsState, model: &'a Model) -> Element<'a, TagsMessage, AppTheme> {
    let left_panel = build_left_panel(state, model);

    let right_content: Element<'a, TagsMessage, AppTheme> =
        if let Some(tag_id) = state.pending_delete {
            build_delete_confirmation(state, tag_id, model)
        } else if let Some(ref creation) = state.creation {
            build_create_form(creation, model)
        } else if let Some(tag_id) = state.selected {
            build_detail_panel(state, tag_id, model)
        } else {
            placeholder_view()
        };

    let right_panel = Container::new(right_content)
        .width(iced::Length::FillPortion(6))
        .height(iced::Length::Fill)
        .style(card);

    Row::new()
        .push(left_panel)
        .push(right_panel)
        .spacing(Spacing::DEFAULT.xs2)
        .height(iced::Length::Fill)
        .into()
}

fn build_left_panel<'a>(
    state: &'a TagsState,
    model: &'a Model,
) -> Element<'a, TagsMessage, AppTheme> {
    let search_bar = build_search_bar(state);

    let visible_ids = if state.search.is_empty() {
        None
    } else {
        Some(state.collect_matching_ids(&model.tag_registry))
    };

    let mut items: Vec<Element<'a, TagsMessage, AppTheme>> = Vec::new();
    collect_tag_nodes(state, model, visible_ids.as_ref(), &mut items, 0, None);

    let tree: Element<'a, TagsMessage, AppTheme> = if items.is_empty() {
        text("No tags found")
            .size(FontSize::Body.px())
            .style(txt::secondary)
            .into()
    } else {
        Column::with_children(items)
            .spacing(Spacing::DEFAULT.xs)
            .into()
    };

    Column::new()
        .push(search_bar)
        .push(rule::horizontal(1))
        .push(scrollable(tree).height(iced::Length::Fill))
        .push(
            Button::new(text("+ New Tag").size(FontSize::Body.px()))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .width(iced::Length::Fill)
                .on_press(TagsMessage::NewTagStarted),
        )
        .spacing(Spacing::DEFAULT.s2)
        .padding(Spacing::DEFAULT.s2)
        .width(iced::Length::FillPortion(4))
        .into()
}

fn build_search_bar<'a>(state: &'a TagsState) -> Element<'a, TagsMessage, AppTheme> {
    let search_input = AdvancedInput::new("Search tags...")
        .value(&state.search)
        .on_input(TagsMessage::SearchQueryChanged)
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let mut row = Row::new()
        .push(Element::new(search_input))
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    if !state.search.is_empty() {
        row = row.push(
            Button::new(text("Clear").size(FontSize::Footnote.px()))
                .style(button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(TagsMessage::SearchCleared),
        );
    }

    row = row.push(
        Button::new(text("Expand All").size(FontSize::Footnote.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(TagsMessage::ExpandAll),
    );
    row = row.push(
        Button::new(text("Collapse All").size(FontSize::Footnote.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(TagsMessage::CollapseAll),
    );

    row.into()
}

/// Recursively collect tag nodes into the items vector.
fn collect_tag_nodes<'a>(
    state: &'a TagsState,
    model: &'a Model,
    visible_ids: Option<&HashSet<TagId>>,
    items: &mut Vec<Element<'a, TagsMessage, AppTheme>>,
    depth: u32,
    parent_id: Option<TagId>,
) {
    for (id, tag) in model.tag_registry.iter() {
        if tag.parent_id != parent_id {
            continue;
        }

        let is_visible = visible_ids.map(|set| set.contains(id)).unwrap_or(true);
        let has_visible_children = if let Some(set) = visible_ids {
            tag.children_ids.iter().any(|cid| set.contains(cid))
        } else {
            !tag.children_ids.is_empty()
        };

        if !is_visible && !has_visible_children {
            continue;
        }

        let is_renaming = state
            .renaming
            .as_ref()
            .map(|(rid, _)| *rid == *id)
            .unwrap_or(false);

        let row = build_tag_row(*id, tag, depth, is_renaming, state, model);
        items.push(row);

        if state.expanded.contains(id) || (state.search.is_empty() && visible_ids.is_none()) {
            collect_tag_nodes(state, model, visible_ids, items, depth + 1, Some(*id));
        }
    }
}

fn build_tag_row<'a>(
    tag_id: TagId,
    tag: &'a Tag,
    depth: u32,
    is_renaming: bool,
    state: &'a TagsState,
    model: &'a Model,
) -> Element<'a, TagsMessage, AppTheme> {
    let is_selected = state.selected == Some(tag_id);
    let meaning_count = state.get_meaning_count(tag_id, &model.meaning_registry);
    let has_children = !tag.children_ids.is_empty();
    let is_expanded = state.expanded.contains(&tag_id);

    let indent = iced::Length::Fixed(depth as f32 * 16.0);

    // Expand/collapse button
    let expand_btn: Element<'a, TagsMessage, AppTheme> = if has_children {
        let icon = if is_expanded { "▼" } else { "▶" };
        Button::new(text(icon).size(FontSize::Footnote.px()))
            .style(button::tertiary)
            .padding([0.0, 2.0])
            .on_press(if is_expanded {
                TagsMessage::TagCollapsed(tag_id)
            } else {
                TagsMessage::TagExpanded(tag_id)
            })
            .into()
    } else {
        text("  ").size(FontSize::Footnote.px()).into()
    };

    // Tag name or rename input
    let name_element: Element<'a, TagsMessage, AppTheme> = if is_renaming {
        if let Some((_, ref buffer)) = state.renaming {
            Element::new(
                AdvancedInput::new("")
                    .value(buffer)
                    .on_input(TagsMessage::RenameChanged)
                    .width(iced::Length::Fixed(120.0))
                    .padding(Spacing::DEFAULT.xs),
            )
        } else {
            text(&tag.name).size(FontSize::Body.px()).into()
        }
    } else {
        text(&tag.name).size(FontSize::Body.px()).into()
    };

    // Meaning count badge
    let count_element: Element<'a, TagsMessage, AppTheme> = if meaning_count > 0 {
        text(format!("({})", meaning_count))
            .size(FontSize::Footnote.px())
            .style(txt::secondary)
            .into()
    } else {
        text("").size(FontSize::Footnote.px()).into()
    };

    let mut row = Row::new()
        .push(Container::new(text("")).width(indent))
        .push(expand_btn)
        .push(name_element)
        .push(count_element)
        .spacing(Spacing::DEFAULT.xs)
        .align_y(iced::Alignment::Center);

    // Action buttons for selected tag
    if is_selected && !is_renaming {
        row = row.push(
            Button::new(text("Edit").size(FontSize::Footnote.px()))
                .style(button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(TagsMessage::RenameStarted(tag_id)),
        );
    }

    // Rename save/cancel buttons
    if is_renaming {
        row = row.push(
            Button::new(text("Save").size(FontSize::Footnote.px()))
                .style(button::primary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(TagsMessage::RenameSaved(tag_id)),
        );
        row = row.push(
            Button::new(text("Cancel").size(FontSize::Footnote.px()))
                .style(button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(TagsMessage::RenameCancelled),
        );
    }

    let element: Element<'a, TagsMessage, AppTheme> = if is_selected {
        Container::new(row)
            .padding([Spacing::DEFAULT.xs, Spacing::DEFAULT.s])
            .style(card)
            .width(iced::Length::Fill)
            .into()
    } else {
        Container::new(row)
            .padding([Spacing::DEFAULT.xs, Spacing::DEFAULT.s])
            .width(iced::Length::Fill)
            .into()
    };

    // Make the row clickable for selection (except when renaming)
    if !is_renaming {
        Button::new(element)
            .style(button::tertiary)
            .width(iced::Length::Fill)
            .on_press(TagsMessage::TagSelected(tag_id))
            .into()
    } else {
        element
    }
}

fn build_detail_panel<'a>(
    state: &'a TagsState,
    tag_id: TagId,
    model: &'a Model,
) -> Element<'a, TagsMessage, AppTheme> {
    let tag = match model.tag_registry.get(tag_id) {
        Some(t) => t,
        None => return placeholder_view(),
    };

    let meaning_count = state.get_meaning_count(tag_id, &model.meaning_registry);

    // Build parent path string
    let parent_path = build_parent_path(tag.parent_id, model);

    let header = Row::new()
        .push(text(&tag.name).size(FontSize::Heading.px()))
        .push(iced::widget::Space::new().width(iced::Length::Fill))
        .push(
            Button::new(text("✕").size(FontSize::Footnote.px()))
                .style(button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(TagsMessage::DetailClosed),
        )
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    let mut content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(rule::horizontal(1));

    if !parent_path.is_empty() {
        content = content.push(
            Row::new()
                .push(text("Parent: ").size(FontSize::Body.px()))
                .push(text(parent_path).size(FontSize::Body.px()))
                .spacing(Spacing::DEFAULT.s),
        );
    }

    content = content.push(
        Row::new()
            .push(text("Meanings: ").size(FontSize::Body.px()))
            .push(text(format!("{}", meaning_count)).size(FontSize::Body.px()))
            .spacing(Spacing::DEFAULT.s),
    );

    if meaning_count > 0 {
        content = content.push(
            Button::new(text("View Meanings").size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(TagsMessage::NavigateToMeanings(tag_id)),
        );
    }

    content = content.push(rule::horizontal(1));

    // Actions
    let actions = Row::new()
        .spacing(Spacing::DEFAULT.s)
        .push(
            Button::new(text("Rename").size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(TagsMessage::RenameStarted(tag_id)),
        )
        .push(
            Button::new(text("Reparent").size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(TagsMessage::ReparentStarted(tag_id)),
        )
        .push(
            Button::new(text("Delete").size(FontSize::Body.px()))
                .style(button::danger)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(TagsMessage::DeleteRequested(tag_id)),
        );

    content = content.push(text("Actions").size(FontSize::Title.px()));
    content = content.push(actions);

    // Reparent form
    if state.reparenting == Some(tag_id) {
        content = content.push(rule::horizontal(1));
        content = content.push(build_reparent_form(tag_id, model));
    }

    Container::new(content)
        .padding(Spacing::DEFAULT.l)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

fn build_reparent_form<'a>(tag_id: TagId, model: &Model) -> Element<'a, TagsMessage, AppTheme> {
    let current_tag = match model.tag_registry.get(tag_id) {
        Some(t) => t,
        None => return placeholder_view(),
    };

    // Build parent options: None (root) + all tags except self and descendants
    let mut options: Vec<ParentOption> = vec![ParentOption {
        id: None,
        label: "(Root - no parent)".to_string(),
    }];

    for (id, _tag) in model.tag_registry.iter() {
        if *id == tag_id {
            continue;
        }
        // Skip descendants to prevent circular references
        if is_descendant(tag_id, *id, model) {
            continue;
        }
        let path = build_full_path(*id, model);
        options.push(ParentOption {
            id: Some(*id),
            label: path,
        });
    }

    let current = options
        .iter()
        .find(|o| o.id == current_tag.parent_id)
        .cloned();

    Column::new()
        .spacing(Spacing::DEFAULT.s)
        .push(text("Select New Parent").size(FontSize::Title.px()))
        .push(
            Row::new()
                .push(text("Parent:").size(FontSize::Body.px()))
                .push(
                    PickList::new(options, current, |po| TagsMessage::ReparentChanged(po.id))
                        .width(iced::Length::Fixed(200.0)),
                )
                .spacing(Spacing::DEFAULT.s)
                .align_y(iced::Alignment::Center),
        )
        .into()
}

fn build_create_form<'a>(
    creation: &TagCreationState,
    model: &Model,
) -> Element<'a, TagsMessage, AppTheme> {
    let name_input = AdvancedInput::new("Tag name")
        .value(&creation.name)
        .on_input(TagsMessage::NewTagNameChanged)
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    // Parent options
    let mut options: Vec<ParentOption> = vec![ParentOption {
        id: None,
        label: "(Root - no parent)".to_string(),
    }];

    for (id, _tag) in model.tag_registry.iter() {
        let path = build_full_path(*id, model);
        options.push(ParentOption {
            id: Some(*id),
            label: path,
        });
    }

    let current = options.iter().find(|o| o.id == creation.parent_id).cloned();

    let parent_picker = Row::new()
        .push(text("Parent:").size(FontSize::Body.px()))
        .push(
            PickList::new(options, current, |po| {
                TagsMessage::NewTagParentChanged(po.id)
            })
            .width(iced::Length::Fixed(200.0)),
        )
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    let buttons = Row::new()
        .spacing(Spacing::DEFAULT.s)
        .push(
            Button::new(text("Save").size(FontSize::Body.px()))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(TagsMessage::NewTagSaved),
        )
        .push(
            Button::new(text("Cancel").size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(TagsMessage::NewTagCancelled),
        );

    Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(text("New Tag").size(FontSize::Heading.px()))
        .push(Element::new(name_input))
        .push(parent_picker)
        .push(buttons)
        .padding(Spacing::DEFAULT.l)
        .into()
}

fn build_delete_confirmation<'a>(
    state: &TagsState,
    tag_id: TagId,
    model: &Model,
) -> Element<'a, TagsMessage, AppTheme> {
    let tag = match model.tag_registry.get(tag_id) {
        Some(t) => t,
        None => return placeholder_view(),
    };

    let descendant_count = count_descendants(tag_id, model);
    let meaning_count = state.get_meaning_count(tag_id, &model.meaning_registry);

    let warning_text = if descendant_count > 0 {
        format!(
            "This will delete \"{}\" and its {} child tag(s). Associated meanings will be unlinked.",
            tag.name, descendant_count
        )
    } else if meaning_count > 0 {
        format!(
            "This will delete \"{}\" and unlink it from {} meaning(s).",
            tag.name, meaning_count
        )
    } else {
        format!("Delete \"{}\"?", tag.name)
    };

    Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(text("Confirm Deletion").size(FontSize::Heading.px()))
        .push(
            Container::new(text(warning_text).size(FontSize::Body.px()))
                .padding(Spacing::DEFAULT.s)
                .style(|theme: &AppTheme| {
                    let colors = theme.colors();
                    iced::widget::container::Style {
                        background: Some(colors.functional.danger.w50().into()),
                        border: iced::Border {
                            color: colors.functional.danger.w200(),
                            width: 1.0,
                            radius: Spacing::DEFAULT.xs.into(),
                        },
                        ..Default::default()
                    }
                }),
        )
        .push(
            Row::new()
                .spacing(Spacing::DEFAULT.s)
                .push(
                    Button::new(text("Delete").size(FontSize::Body.px()))
                        .style(button::danger)
                        .padding(ButtonSize::Standard.to_iced_padding())
                        .on_press(TagsMessage::DeleteConfirmed(tag_id)),
                )
                .push(
                    Button::new(text("Cancel").size(FontSize::Body.px()))
                        .style(button::secondary)
                        .padding(ButtonSize::Standard.to_iced_padding())
                        .on_press(TagsMessage::DeleteCancelled),
                ),
        )
        .padding(Spacing::DEFAULT.l)
        .into()
}

fn placeholder_view() -> Element<'static, TagsMessage, AppTheme> {
    Column::new().into()
}

/// Build a path string from root to this tag.
fn build_full_path(tag_id: TagId, model: &Model) -> String {
    let mut parts = Vec::new();
    let mut current = Some(tag_id);
    while let Some(id) = current {
        if let Some(tag) = model.tag_registry.get(id) {
            parts.push(tag.name.clone());
            current = tag.parent_id;
        } else {
            break;
        }
    }
    parts.reverse();
    parts.join(" > ")
}

/// Build parent path string (ancestors only, not including self).
fn build_parent_path(parent_id: Option<TagId>, model: &Model) -> String {
    match parent_id {
        Some(id) => build_full_path(id, model),
        None => String::new(),
    }
}

/// Check if `candidate` is a descendant of `ancestor`.
fn is_descendant(ancestor: TagId, candidate: TagId, model: &Model) -> bool {
    if let Some(tag) = model.tag_registry.get(candidate)
        && let Some(parent_id) = tag.parent_id
    {
        if parent_id == ancestor {
            return true;
        }
        return is_descendant(ancestor, parent_id, model);
    }
    false
}

/// Count total descendants of a tag.
fn count_descendants(tag_id: TagId, model: &Model) -> usize {
    let Some(tag) = model.tag_registry.get(tag_id) else {
        return 0;
    };
    let mut count = tag.children_ids.len();
    for child_id in &tag.children_ids {
        count += count_descendants(*child_id, model);
    }
    count
}
