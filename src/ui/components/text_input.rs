use crate::ui::theme::AppTheme;
use iced::widget::text_input::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`TextInput`].
pub fn default(theme: &AppTheme, status: Status) -> Style {
    // let colors = theme.colors();

    // let active = Style {
    //     background: Background::Color(colors.background),
    //     border: Border {
    //         radius: 2.0.into(),
    //         width: 1.0,
    //         color: colors.background,
    //     },
    //     icon: colors.background,
    //     placeholder: colors.background,
    //     value: colors.background,
    //     selection: colors.background,
    // };

    // match status {
    //     Status::Active => active,
    //     Status::Hovered => Style {
    //         border: Border {
    //             color: colors.background,
    //             ..active.border
    //         },
    //         ..active
    //     },
    //     Status::Focused { .. } => Style {
    //         border: Border {
    //             color: colors.background,
    //             ..active.border
    //         },
    //         ..active
    //     },
    //     Status::Disabled => Style {
    //         background: Background::Color(colors.background),
    //         value: active.placeholder,
    //         placeholder: colors.background,
    //         ..active
    //     },
    // }

    iced::widget::text_input::default(&iced::Theme::Light, status)
}
