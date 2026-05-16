//! Status bar — persistent notification bar at the window bottom.

use crate::message::Message;
use crate::ui::notification::{Notification, NotificationLevel};
use crate::ui::theme::{AppTheme, Spacing};
use crate::ui::widgets::button;
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{Alignment, Element, Length};

/// Renders a persistent status bar. Shows all active notifications stacked.
pub fn status_bar<'a>(notifications: &'a [Notification]) -> Element<'a, Message, AppTheme> {
    let notification_spacing = Spacing::DEFAULT.xxs;
    let padding = [Spacing::DEFAULT.xs, Spacing::DEFAULT.m];
    let height = 28.0;

    if notifications.is_empty() {
        return Container::new(Row::new())
            .style(|theme: &AppTheme| empty_style(theme))
            .width(Length::Fill)
            .height(height)
            .into();
    }

    let items: Vec<Element<'a, Message, AppTheme>> = notifications
        .iter()
        .map(|n| {
            let icon = match n.level {
                NotificationLevel::Error => "\u{2715}",
                NotificationLevel::Warning => "\u{26A0}",
                NotificationLevel::Info => "\u{2139}",
            };

            Container::new(
                Row::new()
                    .push(Text::new(icon))
                    .push(Text::new(&n.message).width(Length::Fill))
                    .push(
                        Button::new(Text::new("\u{2715}"))
                            .style(button::secondary)
                            .padding(Spacing::DEFAULT.xs)
                            .on_press(Message::DismissNotification(n.id)),
                    )
                    .spacing(notification_spacing)
                    .align_y(Alignment::Center),
            )
            .style(move |theme: &AppTheme| status_style(theme, n.level))
            .padding(padding)
            .width(Length::Fill)
            .height(height)
            .align_y(Alignment::Center)
            .into()
        })
        .collect();

    Column::with_children(items).into()
}

fn status_style(theme: &AppTheme, level: NotificationLevel) -> iced::widget::container::Style {
    let colors = theme.colors();
    let (bg, border) = match level {
        NotificationLevel::Error => (
            colors.functional.danger.w200(),
            colors.functional.danger.w600(),
        ),
        NotificationLevel::Warning => (
            colors.functional.warning.w200(),
            colors.functional.warning.w600(),
        ),
        NotificationLevel::Info => (colors.functional.info.w200(), colors.functional.info.w600()),
    };

    iced::widget::container::Style {
        background: Some(bg.into()),
        border: iced::Border {
            color: border,
            width: 1.0,
            radius: 0.0.into(),
        },
        text_color: Some(colors.neutral.w900()),
        ..Default::default()
    }
}

fn empty_style(theme: &AppTheme) -> iced::widget::container::Style {
    let colors = theme.colors();
    iced::widget::container::Style {
        background: Some(colors.semantic.surface.raised.into()),
        border: iced::Border {
            color: colors.semantic.border.default,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}
