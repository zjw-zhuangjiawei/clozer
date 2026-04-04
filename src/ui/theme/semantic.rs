//! Semantic layer - High-level semantic color definitions
//!
//! Semantic layer provides colors with clear meaning for components:
//! - text.* for text colors
//! - surface.* for background/surface colors
//! - border.* for border colors
//! - interactive.* for interactive element colors

use crate::ui::theme::color::{Color, ColorMode, ColorScale, FunctionalPalette};
use crate::ui::theme::role::{BackgroundRole, BorderRole, ForegroundRole, InteractiveRole};

#[derive(Debug, Clone)]
pub struct TextSemantic {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub disabled: Color,
    pub inverse: Color,
    pub link: Color,
    pub error: Color,
}

#[derive(Debug, Clone)]
pub struct SurfaceSemantic {
    pub base: Color,
    pub raised: Color,
    pub elevated: Color,
    pub overlay: Color,
}

#[derive(Debug, Clone)]
pub struct BorderSemantic {
    pub default: Color,
    pub hover: Color,
    pub focus: Color,
    pub disabled: Color,
}

#[derive(Debug, Clone)]
pub struct InteractiveSemantic {
    pub primary: Color,
    pub primary_hover: Color,
    pub secondary: Color,
    pub secondary_hover: Color,
    pub danger: Color,
    pub danger_hover: Color,
    pub link: Color,
}

#[derive(Debug, Clone)]
pub struct SemanticPalette {
    pub text: TextSemantic,
    pub surface: SurfaceSemantic,
    pub border: BorderSemantic,
    pub interactive: InteractiveSemantic,
}

pub struct SemanticPaletteBuilder {
    mode: ColorMode,
    primary: ColorScale,
    neutral: ColorScale,
    functional: FunctionalPalette,
}

impl SemanticPaletteBuilder {
    pub fn new(
        mode: ColorMode,
        primary: ColorScale,
        neutral: ColorScale,
        functional: FunctionalPalette,
    ) -> Self {
        Self {
            mode,
            primary,
            neutral,
            functional,
        }
    }

    pub fn build(self) -> SemanticPalette {
        let mode = self.mode;

        let text = TextSemantic {
            primary: ForegroundRole::primary(&self.neutral, mode),
            secondary: ForegroundRole::secondary(&self.neutral, mode),
            tertiary: ForegroundRole::tertiary(&self.neutral, mode),
            disabled: ForegroundRole::disabled(&self.neutral, mode),
            inverse: match mode {
                ColorMode::Light => BackgroundRole::base(&self.neutral, mode),
                ColorMode::Dark => BackgroundRole::base(&self.neutral, mode),
            },
            link: InteractiveRole::link(&self.primary, mode),
            error: InteractiveRole::danger(&self.functional, mode),
        };

        let surface = SurfaceSemantic {
            base: BackgroundRole::base(&self.neutral, mode),
            raised: BackgroundRole::raised(&self.neutral, mode),
            elevated: BackgroundRole::elevated(&self.neutral, mode),
            overlay: BackgroundRole::overlay(&self.neutral, mode),
        };

        let border = BorderSemantic {
            default: BorderRole::default(&self.neutral, mode),
            hover: BorderRole::strong(&self.neutral, mode),
            focus: BorderRole::focus(&self.primary, mode),
            disabled: match mode {
                ColorMode::Light => self.neutral.w100(),
                ColorMode::Dark => self.neutral.w700(),
            },
        };

        let interactive = InteractiveSemantic {
            primary: InteractiveRole::primary(&self.primary, mode),
            primary_hover: InteractiveRole::primary_hover(&self.primary, mode),
            secondary: InteractiveRole::secondary(&self.neutral, mode),
            secondary_hover: InteractiveRole::secondary_hover(&self.neutral, mode),
            danger: InteractiveRole::danger(&self.functional, mode),
            danger_hover: InteractiveRole::danger_hover(&self.functional, mode),
            link: InteractiveRole::link(&self.primary, mode),
        };

        SemanticPalette {
            text,
            surface,
            border,
            interactive,
        }
    }
}
