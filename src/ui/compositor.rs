use crate::message::Message;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::layout::{LayoutConfig, LayoutMode, adaptive_layout, breakpoint::Breakpoint};
use crate::ui::nav::NavItem;
use crate::ui::notification::{Notification, NotificationLevel};
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

fn notification_style(
    level: NotificationLevel,
) -> impl Fn(&AppTheme) -> iced::widget::container::Style {
    move |theme: &AppTheme| {
        let colors = theme.colors();
        let bg = match level {
            NotificationLevel::Error => colors.functional.danger.w100(),
            NotificationLevel::Warning => colors.functional.warning.w100(),
            NotificationLevel::Info => colors.functional.info.w100(),
        };
        iced::widget::container::Style {
            background: Some(bg.into()),
            border: iced::Border {
                color: colors.semantic.border.default,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

fn render_notifications<'a>(state: &'a UiState) -> Option<Element<'a, Message, AppTheme>> {
    if state.notifications.is_empty() {
        return None;
    }

    let banners: Vec<Element<'a, Message, AppTheme>> = state
        .notifications
        .iter()
        .map(|n| {
            let icon = match n.level {
                NotificationLevel::Error => "\u{2715}",
                NotificationLevel::Warning => "\u{26A0}",
                NotificationLevel::Info => "\u{2139}",
            };
            let id = n.id;
            let level = n.level;

            iced::widget::container(
                iced::widget::row![
                    iced::widget::text(icon).style(move |theme: &AppTheme| {
                        let tc = match level {
                            NotificationLevel::Error => theme.colors().functional.danger.w700(),
                            NotificationLevel::Warning => theme.colors().functional.warning.w700(),
                            NotificationLevel::Info => theme.colors().functional.info.w700(),
                        };
                        iced::widget::text::Style { color: Some(tc) }
                    }),
                    iced::widget::text(&n.message)
                        .width(iced::Length::Fill)
                        .style(move |theme: &AppTheme| {
                            let tc = match level {
                                NotificationLevel::Error => theme.colors().functional.danger.w700(),
                                NotificationLevel::Warning => {
                                    theme.colors().functional.warning.w700()
                                }
                                NotificationLevel::Info => theme.colors().functional.info.w700(),
                            };
                            iced::widget::text::Style { color: Some(tc) }
                        }),
                    iced::widget::text(&n.message)
                        .width(iced::Length::Fill)
                        .style({
                            let level = level;
                            move |theme: &AppTheme| {
                                let tc = match level {
                                    NotificationLevel::Error => {
                                        theme.colors().functional.danger.w700()
                                    }
                                    NotificationLevel::Warning => {
                                        theme.colors().functional.warning.w700()
                                    }
                                    NotificationLevel::Info => {
                                        theme.colors().functional.info.w700()
                                    }
                                };
                                iced::widget::text::Style { color: Some(tc) }
                            }
                        }),
                    iced::widget::button(iced::widget::text("\u{2715}"))
                        .style(button::secondary)
                        .padding([2.0, 6.0])
                        .on_press(Message::DismissNotification(id)),
                ]
                .spacing(Spacing::DEFAULT.xs)
                .align_y(iced::Alignment::Center),
            )
            .style(notification_style(level))
            .padding([Spacing::DEFAULT.xs, Spacing::DEFAULT.s])
            .into()
        })
        .collect();

    Some(
        iced::widget::Column::with_children(banners)
            .spacing(Spacing::DEFAULT.xs)
            .into(),
    )
}

fn responsive_panel<'a, M: 'a>(
    content: Element<'a, M, AppTheme>,
    breakpoint: Breakpoint,
    left_ratio: f32,
) -> Element<'a, M, AppTheme> {
    let spacing = Spacing::DEFAULT.l2;
    if breakpoint.is_single_column() {
        iced::widget::column![content]
            .spacing(spacing)
            .padding(spacing)
            .into()
    } else {
        iced::widget::row![
            iced::widget::container(content)
                .width(FillPortion((left_ratio * 100.0) as u16))
                .padding(spacing)
        ]
        .spacing(spacing)
        .into()
    }
}

pub fn view<'a>(state: &'a UiState, model: &'a Model) -> Element<'a, Message, AppTheme> {
    let breakpoint = Breakpoint::from_width(state.window_width as f32);

    let nav_spacing = if breakpoint.is_single_column() {
        Spacing::DEFAULT.xs2
    } else {
        Spacing::DEFAULT.s2
    };
    let nav_buttons: Vec<Element<'a, Message, AppTheme>> = [
        NavItem::Words,
        NavItem::Queue,
        NavItem::Tags,
        NavItem::Settings,
    ]
    .iter()
    .map(|item| {
        let is_active = state.current_view == *item;
        let label = item.label();
        let btn = iced::widget::button(iced::widget::text(label))
            .style(if is_active {
                button::primary
            } else {
                button::secondary
            })
            .on_press(Message::Navigate(*item));
        btn.into()
    })
    .collect();

    let nav_bar = iced::widget::Row::with_children(nav_buttons).spacing(nav_spacing);
    let (left_ratio, _right_ratio) = breakpoint.column_ratio();

    let content: Element<'a, Message, AppTheme> = match state.current_view {
        NavItem::Words => responsive_panel(
            crate::ui::words::explorer::view(&state.words, model, breakpoint).map(Message::Words),
            breakpoint,
            left_ratio,
        ),
        NavItem::Queue => responsive_panel(
            crate::ui::queue::view(model).map(Message::Queue),
            breakpoint,
            left_ratio,
        ),
        NavItem::Tags => responsive_panel(
            crate::ui::tags::view(&state.tags, model).map(Message::Tags),
            breakpoint,
            left_ratio,
        ),
        NavItem::Settings => responsive_panel(
            crate::ui::settings::view::view(&state.settings, model).map(Message::Settings),
            breakpoint,
            left_ratio,
        ),
    };

    let layout_config = get_layout_config();
    let base = match layout_config.mode {
        LayoutMode::Adaptive => adaptive_layout(nav_bar.into(), content, breakpoint),
        _ => iced::widget::column![nav_bar, content].into(),
    };

    match render_notifications(state) {
        Some(banners) => iced::widget::column![banners, base]
            .spacing(Spacing::DEFAULT.xs)
            .into(),
        None => base,
    }
}

pub fn update_words(
    state: &mut WordsState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<Message> {
    crate::ui::words::update(state, message, model).map(|msg| match msg {
        WordsMessage::ExportFailed(err) => {
            Message::PushNotification(Notification::error(0, format!("Export failed: {}", err)))
        }
        other => Message::Words(other),
    })
}

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
