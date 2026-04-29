//! UI compositor — composes words, queue, and settings panels into a single view.
//!
//! Acts as a thin coordination layer: builds nav bar, routes to active panel,
//! and applies adaptive layout. Panel-specific state lives in each panel's module.

use crate::message::Message;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::layout::{LayoutConfig, LayoutMode, adaptive_layout, breakpoint::Breakpoint};
use crate::ui::nav::NavItem;
use crate::ui::settings::state::SettingsState;
use crate::ui::state::UiState;
use crate::ui::theme::Spacing;
use crate::ui::widgets::button;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::{Element, FillPortion, Task};

static LAYOUT_CONFIG: std::sync::OnceLock<LayoutConfig> = std::sync::OnceLock::new();

fn get_layout_config() -> &'static LayoutConfig {
    LAYOUT_CONFIG.get_or_init(LayoutConfig::adaptive)
}

pub fn view<'a>(state: &'a UiState, model: &'a Model) -> Element<'a, Message, AppTheme> {
    let breakpoint = Breakpoint::from_width(state.window_width as f32);

    let nav_spacing = if breakpoint.is_single_column() {
        Spacing::DEFAULT.xs2
    } else {
        Spacing::DEFAULT.s2
    };
    let nav_buttons: Vec<Element<'a, Message, AppTheme>> =
        [NavItem::Words, NavItem::Queue, NavItem::Settings]
            .iter()
            .map(|item| {
                let is_active = state.current_view == *item;
                let label = item.label();
                let button = iced::widget::button(iced::widget::text(label))
                    .style(if is_active {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .on_press(Message::Navigate(*item));
                button.into()
            })
            .collect();

    let nav_bar = iced::widget::Row::with_children(nav_buttons).spacing(nav_spacing);
    let (left_ratio, _right_ratio) = breakpoint.column_ratio();

    // Content based on current navigation view
    let content: Element<'a, Message, AppTheme> = match state.current_view {
        NavItem::Words => {
            let left_panel = crate::ui::words::view(&state.words, model, state.theme, breakpoint)
                .map(Message::Words);
            if breakpoint.is_single_column() {
                iced::widget::column![left_panel]
                    .spacing(Spacing::DEFAULT.l2)
                    .padding(Spacing::DEFAULT.l2)
                    .into()
            } else {
                iced::widget::row![
                    iced::widget::container(left_panel)
                        .width(FillPortion((left_ratio * 100.0) as u16))
                        .padding(Spacing::DEFAULT.l2),
                ]
                .spacing(Spacing::DEFAULT.l2)
                .into()
            }
        }
        NavItem::Queue => {
            let queue_panel = crate::ui::queue::view(model, state.theme).map(Message::Queue);
            if breakpoint.is_single_column() {
                iced::widget::column![queue_panel]
                    .spacing(Spacing::DEFAULT.l2)
                    .padding(Spacing::DEFAULT.l2)
                    .into()
            } else {
                iced::widget::row![
                    iced::widget::container(queue_panel)
                        .width(FillPortion((left_ratio * 100.0) as u16))
                        .padding(Spacing::DEFAULT.l2),
                ]
                .spacing(Spacing::DEFAULT.l2)
                .into()
            }
        }
        NavItem::Settings => {
            let settings_panel = crate::ui::settings::view::view(model).map(Message::Settings);
            if breakpoint.is_single_column() {
                iced::widget::column![settings_panel]
                    .spacing(Spacing::DEFAULT.l2)
                    .padding(Spacing::DEFAULT.l2)
                    .into()
            } else {
                iced::widget::row![
                    iced::widget::container(settings_panel)
                        .width(FillPortion((left_ratio * 100.0) as u16))
                        .padding(Spacing::DEFAULT.l2),
                ]
                .spacing(Spacing::DEFAULT.l2)
                .into()
            }
        }
    };

    // Use layout system based on configuration
    let layout_config = get_layout_config();
    match layout_config.mode {
        LayoutMode::Adaptive => adaptive_layout(nav_bar.into(), content, breakpoint),
        _ => iced::widget::column![nav_bar, content].into(),
    }
}

/// Handles words panel update — takes only WordsState, not full UiState.
pub fn update_words(
    state: &mut WordsState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<Message> {
    crate::ui::words::update(state, message, model).map(Message::Words)
}

/// Handles settings panel update — takes only SettingsState, not full UiState.
pub fn update_settings(
    state: &mut SettingsState,
    message: crate::ui::settings::SettingsMessage,
    _model: &mut Model,
) -> Task<Message> {
    use crate::ui::settings::SettingsMessage;
    match message {
        SettingsMessage::ThemeChanged(theme) => {
            Task::done(crate::message::Message::ThemeChanged(theme))
        }
        _ => crate::ui::settings::handlers::update(state, message, _model).map(Message::Settings),
    }
}
