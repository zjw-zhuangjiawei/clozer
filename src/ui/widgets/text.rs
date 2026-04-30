use crate::ui::AppTheme;
use iced::widget::text::{Catalog, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn primary(theme: &AppTheme) -> Style {
    Style {
        color: Some(theme.colors().semantic.text.primary),
    }
}

pub fn secondary(theme: &AppTheme) -> Style {
    Style {
        color: Some(theme.colors().semantic.text.secondary),
    }
}

pub fn tertiary(theme: &AppTheme) -> Style {
    Style {
        color: Some(theme.colors().semantic.text.tertiary),
    }
}

pub fn primary_alt(theme: &AppTheme) -> Style {
    Style {
        color: Some(theme.colors().semantic.interactive.primary),
    }
}

pub fn success(theme: &AppTheme) -> Style {
    Style {
        color: Some(theme.colors().functional.success.w600()),
    }
}

pub fn error(theme: &AppTheme) -> Style {
    Style {
        color: Some(theme.colors().semantic.text.error),
    }
}
