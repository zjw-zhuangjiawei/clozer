//! Left sidebar navigation component.
//!
//! Vertical tab strip with icon + label. Collapses to icon-only
//! on medium screens, recedes to bottom tab bar on small screens.

use crate::message::Message;
use crate::ui::layout::breakpoint::Breakpoint;
use crate::ui::nav::NavItem;
use crate::ui::state::UiState;
use crate::ui::theme::{AppTheme, BorderRadiusValues, Spacing};
use iced::widget::{Button, Column, Row, Space, Text};
use iced::{Alignment, Element, Length};

/// Build the full sidebar panel for the current state.
/// Returns a no-op element if the bottom bar is being used instead.
pub fn sidebar<'a>(state: &'a UiState, breakpoint: Breakpoint) -> Element<'a, Message, AppTheme> {
    if breakpoint.use_bottom_bar() {
        return Space::new().into();
    }

    let expanded = breakpoint.sidebar_expanded();
    let width = breakpoint.sidebar_panel_width();

    let items: Vec<Element<'a, Message, AppTheme>> = NavItem::all()
        .iter()
        .map(|&item| {
            let is_active = state.current_view == item;
            sidebar_item(item, is_active, expanded)
        })
        .collect();

    Column::with_children(items)
        .width(Length::Fixed(width))
        .spacing(Spacing::DEFAULT.xs)
        .padding(Spacing::DEFAULT.s)
        .into()
}

/// Build a single sidebar navigation item.
fn sidebar_item<'a>(
    item: NavItem,
    is_active: bool,
    expanded: bool,
) -> Element<'a, Message, AppTheme> {
    let icon = match item {
        NavItem::Words => "\u{1F4DD}",
        NavItem::Queue => "\u{1F4CB}",
        NavItem::Tags => "\u{1F3F7}\u{FE0F}",
        NavItem::Settings => "\u{2699}\u{FE0F}",
    };
    let label = item.label();

    let content: Element<'a, Message, AppTheme> = if expanded {
        Row::new()
            .push(Text::new(icon).size(18.0))
            .push(Text::new(label).size(14.0))
            .spacing(Spacing::DEFAULT.s)
            .align_y(Alignment::Center)
            .into()
    } else {
        Column::new()
            .push(Text::new(icon).size(20.0))
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    };

    let indicator = Space::new().width(Length::Fixed(3.0)).height(Length::Fill);

    Button::new(Row::new().push(indicator).push(content))
        .style(sidebar_button_style(is_active))
        .width(Length::Fill)
        .on_press(Message::Navigate(item))
        .into()
}

/// Style function for sidebar navigation buttons.
fn sidebar_button_style(
    is_active: bool,
) -> impl Fn(&AppTheme, iced::widget::button::Status) -> iced::widget::button::Style {
    move |theme: &AppTheme, status: iced::widget::button::Status| {
        let colors = theme.colors();
        let semantic = &colors.semantic;

        let bg = match (is_active, status) {
            (true, _) => semantic.interactive.secondary_hover,
            (false, iced::widget::button::Status::Hovered) => semantic.interactive.secondary,
            (false, iced::widget::button::Status::Pressed) => semantic.interactive.secondary_hover,
            _ => colors.neutral.w50(),
        };

        iced::widget::button::Style {
            background: Some(bg.into()),
            border: iced::Border {
                color: if is_active {
                    semantic.interactive.primary
                } else {
                    colors.neutral.w50()
                },
                width: 0.0,
                radius: BorderRadiusValues::default().sm.into(),
            },
            text_color: if is_active {
                semantic.text.primary
            } else {
                semantic.text.secondary
            },
            ..Default::default()
        }
    }
}

/// Build the bottom tab bar for very narrow screens.
pub fn bottom_tab_bar<'a>(state: &'a UiState) -> Element<'a, Message, AppTheme> {
    let items: Vec<Element<'a, Message, AppTheme>> = NavItem::all()
        .iter()
        .map(|&item| {
            let is_active = state.current_view == item;
            let icon = match item {
                NavItem::Words => "\u{1F4DD}",
                NavItem::Queue => "\u{1F4CB}",
                NavItem::Tags => "\u{1F3F7}\u{FE0F}",
                NavItem::Settings => "\u{2699}\u{FE0F}",
            };
            let label = item.label();

            let content = Column::new()
                .push(Text::new(icon).size(18.0))
                .push(Text::new(label).size(10.0))
                .align_x(Alignment::Center)
                .spacing(2.0);

            Button::new(content)
                .style(sidebar_button_style(is_active))
                .width(Length::Fill)
                .on_press(Message::Navigate(item))
                .into()
        })
        .collect();

    Row::with_children(items)
        .padding([Spacing::DEFAULT.xs, Spacing::DEFAULT.s])
        .spacing(0.0)
        .into()
}
