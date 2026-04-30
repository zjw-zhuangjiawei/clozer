use crate::ui::theme::AppTheme;
use iced::widget::scrollable::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`Scrollable`].
pub fn default(theme: &AppTheme, _status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    iced::widget::scrollable::Style {
        container: iced::widget::container::Style::default(),
        vertical_rail: iced::widget::scrollable::Rail {
            background: None,
            border: iced::Border {
                radius: 4.0.into(),
                width: 0.0,
                color: semantic.border.disabled,
            },
            scroller: iced::widget::scrollable::Scroller {
                background: semantic.border.default.into(),
                border: iced::Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: semantic.border.default,
                },
            },
        },
        horizontal_rail: iced::widget::scrollable::Rail {
            background: None,
            border: iced::Border {
                radius: 4.0.into(),
                width: 0.0,
                color: semantic.border.disabled,
            },
            scroller: iced::widget::scrollable::Scroller {
                background: semantic.border.default.into(),
                border: iced::Border {
                    radius: 4.0.into(),
                    width: 1.0,
                    color: semantic.border.default,
                },
            },
        },
        gap: None,
        auto_scroll: iced::widget::scrollable::AutoScroll {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            border: iced::Border {
                radius: 4.0.into(),
                width: 0.0,
                color: iced::Color::TRANSPARENT,
            },
            shadow: iced::Shadow::default(),
            icon: iced::Color::TRANSPARENT,
        },
    }
}
