//! Left sidebar navigation component.
//!
//! Vertical tab strip with icon + label. Collapses to icon-only
//! on medium screens, recedes to bottom tab bar on small screens.

use crate::message::Message;
use crate::ui::design_tokens::{FontSize, Spacing, TouchTargetSize};
use crate::ui::layout::breakpoint::Breakpoint;
use crate::ui::nav::NavItem;
use crate::ui::state::UiState;
use crate::ui::theme::{AppTheme, BorderRadiusValues};
use iced::widget::{Button, Column, Container, Row, Space, Text, Tooltip, tooltip};
use iced::{Alignment, Element, Length, Padding};

/// Build the full sidebar panel for the current state.
/// Returns a no-op element if the bottom bar is being used instead.
pub fn sidebar<'a>(state: &'a UiState, breakpoint: Breakpoint) -> Element<'a, Message, AppTheme> {
    if breakpoint.use_bottom_bar() {
        return Space::new().into();
    }

    let expanded = breakpoint.sidebar_expanded();
    let width = breakpoint.sidebar_panel_width();

    // Brand header
    let header = sidebar_header(expanded);

    // Main navigation items
    let main_items: Vec<Element<'a, Message, AppTheme>> = NavItem::main()
        .iter()
        .map(|&item| {
            let is_active = state.current_view == item;
            nav_button(item, is_active, expanded)
        })
        .collect();

    // Secondary navigation items (e.g., Settings)
    let secondary_items: Vec<Element<'a, Message, AppTheme>> = NavItem::secondary()
        .iter()
        .map(|&item| {
            let is_active = state.current_view == item;
            nav_button(item, is_active, expanded)
        })
        .collect();

    Column::new()
        .push(header)
        .push(separator())
        .push(Space::new().height(Length::Fixed(Spacing::DEFAULT.s)))
        .push(Column::with_children(main_items).spacing(Spacing::DEFAULT.xxs))
        .push(Space::new().height(Length::Fill))
        .push(separator())
        .push(Column::with_children(secondary_items).spacing(Spacing::DEFAULT.xxs))
        .push(Space::new().height(Length::Fixed(Spacing::DEFAULT.s)))
        .width(Length::Fixed(width))
        .spacing(0)
        .padding(Padding::from([0.0, Spacing::DEFAULT.xs]))
        .into()
}

/// Brand header at the top of the sidebar.
fn sidebar_header<'a>(expanded: bool) -> Element<'a, Message, AppTheme> {
    if expanded {
        Container::new(
            Text::new("clozer")
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .size(FontSize::Title.px()),
        )
        .padding([Spacing::DEFAULT.l, Spacing::DEFAULT.s])
        .width(Length::Fill)
        .into()
    } else {
        Container::new(
            Text::new("C")
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .size(FontSize::Title.px())
                .align_x(Alignment::Center)
                .width(Length::Fill),
        )
        .padding([Spacing::DEFAULT.l, Spacing::DEFAULT.xs])
        .width(Length::Fill)
        .into()
    }
}

/// Build a single sidebar navigation button.
fn nav_button<'a>(
    item: NavItem,
    is_active: bool,
    expanded: bool,
) -> Element<'a, Message, AppTheme> {
    let icon_size = if expanded { 20.0 } else { 24.0 };

    let icon = nav_icon(item, icon_size);
    let label = item.label();

    let content: Element<'a, Message, AppTheme> = if expanded {
        Row::new()
            .push(icon)
            .push(Text::new(label).size(FontSize::Body.px()).font(iced::Font {
                weight: if is_active {
                    iced::font::Weight::Semibold
                } else {
                    iced::font::Weight::Normal
                },
                ..Default::default()
            }))
            .push(Space::new().width(Length::Fill))
            .push(keyboard_hint(item))
            .spacing(Spacing::DEFAULT.s)
            .align_y(Alignment::Center)
            .into()
    } else {
        Column::new()
            .push(icon)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .into()
    };

    let button = Button::new(content)
        .style(nav_button_style(is_active))
        .width(Length::Fill)
        .height(Length::Fixed(TouchTargetSize::default().recommended))
        .on_press(Message::Navigate(item));

    if expanded {
        button.into()
    } else {
        Tooltip::new(button, label, tooltip::Position::Right).into()
    }
}

/// Emoji icon for a navigation item.
fn nav_icon<'a>(item: NavItem, size: f32) -> Element<'a, Message, AppTheme> {
    let icon_text = match item {
        NavItem::Words => "\u{1F4DD}",
        NavItem::Queue => "\u{1F4CB}",
        NavItem::Tags => "\u{1F3F7}\u{FE0F}",
        NavItem::Settings => "\u{2699}\u{FE0F}",
    };
    Text::new(icon_text)
        .size(size)
        .align_x(Alignment::Center)
        .into()
}

/// Keyboard shortcut hint shown on expanded sidebar items.
fn keyboard_hint<'a>(item: NavItem) -> Element<'a, Message, AppTheme> {
    let shortcut = match item {
        NavItem::Words => "Ctrl+1",
        NavItem::Queue => "Ctrl+2",
        NavItem::Tags => "Ctrl+3",
        NavItem::Settings => "Ctrl+4",
    };
    Text::new(shortcut).size(FontSize::Caption.px()).into()
}

/// Horizontal separator line.
fn separator<'a>() -> Element<'a, Message, AppTheme> {
    Container::new(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
        .style(|theme: &AppTheme| iced::widget::container::Style {
            background: Some(theme.colors().semantic.border.default.into()),
            ..Default::default()
        })
        .width(Length::Fill)
        .padding([0.0, Spacing::DEFAULT.xs])
        .into()
}

/// Style function for sidebar navigation buttons.
fn nav_button_style(
    is_active: bool,
) -> impl Fn(&AppTheme, iced::widget::button::Status) -> iced::widget::button::Style + use<> {
    move |theme: &AppTheme, status: iced::widget::button::Status| {
        let colors = theme.colors();
        let semantic = &colors.semantic;
        let radius = BorderRadiusValues::default();

        let bg = match (is_active, status) {
            (true, _) => semantic.surface.raised,
            (false, iced::widget::button::Status::Hovered) => semantic.interactive.secondary,
            (false, iced::widget::button::Status::Pressed) => semantic.interactive.secondary_hover,
            _ => iced::Color::TRANSPARENT,
        };

        iced::widget::button::Style {
            background: Some(bg.into()),
            border: iced::Border {
                color: if is_active {
                    colors.primary.w500()
                } else {
                    iced::Color::TRANSPARENT
                },
                width: if is_active { 3.0 } else { 0.0 },
                radius: radius.md.into(),
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
            let icon = nav_icon(item, 20.0);
            let label = item.label();

            let content = Column::new()
                .push(icon)
                .push(Text::new(label).size(FontSize::Caption.px()))
                .align_x(Alignment::Center)
                .spacing(Spacing::DEFAULT.xxs);

            Button::new(content)
                .style(nav_button_style(is_active))
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
