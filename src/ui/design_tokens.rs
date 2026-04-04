//! Centralized design tokens for consistent styling across the application.
//!
//! Design tokens are the atomic values that define the visual language:
//! - Colors (semantic with scales)
//! - Typography (scale with ratios)
//! - Spacing (proportional system)
//! - Dimensions (components, borders, radii)
//! - Motion (durations, easings)
//!
//! # Design Principles
//!
//! 1. **Clarity First**: Visual hierarchy > information density
//! 2. **Consistency**: Same function = same style
//! 3. **Progressive Disclosure**: Core info first, details on demand
//! 4. **Accessibility**: WCAG 2.1 AA compliance required
//! 5. **Systematic**: Tokens drive everything, mathematical ratios control proportions

use iced::Color;

pub mod prelude {
    pub use super::{
        BackgroundColors, BorderColors, BorderRadiusValues, CategoryColor, CefrBadgeStyle,
        ColorScale, DesignTokens, DimensionTokens, DurationScale, EasingFunctions, FocusColors,
        FontFamily, FontWeights, IconSizeValues, LineHeights, MotionTokens, RadiusVariant,
        ScaleFactor, ScaleSystem, SemanticColors, Severity, SpacingScale, SpacingValue,
        StatusColors, TextColors, TypographyScale, TypographyVariant,
    };
}

#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub scale: ScaleSystem,
    pub colors: SemanticColors,
    pub typography: TypographyScale,
    pub spacing: SpacingScale,
    pub dimensions: DimensionTokens,
    pub motion: MotionTokens,
}

impl DesignTokens {
    pub fn light() -> Self {
        Self {
            scale: ScaleSystem::default(),
            colors: SemanticColors::light(),
            typography: TypographyScale::default(),
            spacing: SpacingScale::default(),
            dimensions: DimensionTokens::default(),
            motion: MotionTokens::default(),
        }
    }

    pub fn dark() -> Self {
        Self {
            scale: ScaleSystem::default(),
            colors: SemanticColors::dark(),
            typography: TypographyScale::default(),
            spacing: SpacingScale::default(),
            dimensions: DimensionTokens::default(),
            motion: MotionTokens::default(),
        }
    }
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self::light()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScaleSystem {
    pub base_unit: f32,
    pub scale_factor: ScaleFactor,
}

#[derive(Debug, Clone, Copy)]
pub enum ScaleFactor {
    Small,  // 1.125 - subtle scaling
    Medium, // 1.25 - perfect fourth
    Large,  // 1.333 - perfect third
}

impl Default for ScaleSystem {
    fn default() -> Self {
        Self {
            base_unit: 8.0,
            scale_factor: ScaleFactor::Medium,
        }
    }
}

impl ScaleFactor {
    pub fn value(&self) -> f32 {
        match self {
            ScaleFactor::Small => 1.125,
            ScaleFactor::Medium => 1.25,
            ScaleFactor::Large => 1.333,
        }
    }
}

impl ScaleSystem {
    pub fn size(&self, step: i8) -> f32 {
        let factor = self.scale_factor.value();
        let base = self.base_unit;

        if step == 0 {
            return base;
        }

        if step > 0 {
            let mut result = base;
            for _ in 0..step {
                result *= factor;
            }
            result
        } else {
            let mut result = base;
            for _ in 0..(-step) {
                result /= factor;
            }
            result
        }
    }

    pub fn xs(&self) -> f32 {
        self.size(-2)
    } // 4px
    pub fn sm(&self) -> f32 {
        self.size(-1)
    } // 5px
    pub fn md(&self) -> f32 {
        self.size(0)
    } // 8px
    pub fn lg(&self) -> f32 {
        self.size(1)
    } // 10px
    pub fn xl(&self) -> f32 {
        self.size(2)
    } // 13px
    pub fn xxl(&self) -> f32 {
        self.size(3)
    } // 16px
    pub fn xxxl(&self) -> f32 {
        self.size(4)
    } // 20px
}

#[derive(Debug, Clone)]
pub struct SemanticColors {
    pub primary: ColorScale,
    pub secondary: ColorScale,
    pub neutral: ColorScale,
    pub success: ColorScale,
    pub warning: ColorScale,
    pub danger: ColorScale,
    pub info: ColorScale,
    pub background: BackgroundColors,
    pub text: TextColors,
    pub border: BorderColors,
    pub focus: FocusColors,
    pub status: StatusColors,
}

impl SemanticColors {
    pub fn light() -> Self {
        Self {
            primary: ColorScale::new(
                Color::from_rgb(0.95, 0.97, 1.0),  // 50 - lightest
                Color::from_rgb(0.93, 0.96, 1.0),  // 100
                Color::from_rgb(0.86, 0.92, 0.98), // 200
                Color::from_rgb(0.74, 0.85, 0.95), // 300
                Color::from_rgb(0.55, 0.73, 0.88), // 400
                Color::from_rgb(0.2, 0.4, 0.6),    // 500 - base
                Color::from_rgb(0.17, 0.35, 0.53), // 600 - hover
                Color::from_rgb(0.13, 0.27, 0.4),  // 700 - active
                Color::from_rgb(0.1, 0.2, 0.3),    // 900 - darkest
            ),
            secondary: ColorScale::new(
                Color::from_rgb(0.98, 0.98, 0.98),
                Color::from_rgb(0.95, 0.95, 0.95),
                Color::from_rgb(0.88, 0.88, 0.88),
                Color::from_rgb(0.75, 0.75, 0.75),
                Color::from_rgb(0.55, 0.55, 0.55),
                Color::from_rgb(0.45, 0.45, 0.45),
                Color::from_rgb(0.35, 0.35, 0.35),
                Color::from_rgb(0.25, 0.25, 0.25),
                Color::from_rgb(0.15, 0.15, 0.15),
            ),
            neutral: ColorScale::new(
                Color::from_rgb(0.98, 0.98, 0.98),
                Color::from_rgb(0.96, 0.96, 0.96),
                Color::from_rgb(0.92, 0.92, 0.92),
                Color::from_rgb(0.88, 0.88, 0.88),
                Color::from_rgb(0.82, 0.82, 0.82),
                Color::from_rgb(0.6, 0.6, 0.6),
                Color::from_rgb(0.45, 0.45, 0.45),
                Color::from_rgb(0.35, 0.35, 0.35),
                Color::from_rgb(0.1, 0.1, 0.1),
            ),
            success: ColorScale::new(
                Color::from_rgb(0.95, 0.99, 0.96),
                Color::from_rgb(0.87, 0.97, 0.9),
                Color::from_rgb(0.74, 0.92, 0.8),
                Color::from_rgb(0.56, 0.84, 0.66),
                Color::from_rgb(0.38, 0.73, 0.52),
                Color::from_rgb(0.2, 0.6, 0.3),
                Color::from_rgb(0.17, 0.5, 0.27),
                Color::from_rgb(0.13, 0.4, 0.2),
                Color::from_rgb(0.05, 0.25, 0.1),
            ),
            warning: ColorScale::new(
                Color::from_rgb(1.0, 0.98, 0.92),
                Color::from_rgb(1.0, 0.95, 0.84),
                Color::from_rgb(1.0, 0.9, 0.7),
                Color::from_rgb(1.0, 0.82, 0.5),
                Color::from_rgb(1.0, 0.7, 0.3),
                Color::from_rgb(0.9, 0.55, 0.2),
                Color::from_rgb(0.78, 0.47, 0.17),
                Color::from_rgb(0.65, 0.38, 0.13),
                Color::from_rgb(0.45, 0.25, 0.05),
            ),
            danger: ColorScale::new(
                Color::from_rgb(0.99, 0.95, 0.95),
                Color::from_rgb(0.98, 0.9, 0.9),
                Color::from_rgb(0.96, 0.8, 0.8),
                Color::from_rgb(0.92, 0.65, 0.65),
                Color::from_rgb(0.85, 0.45, 0.45),
                Color::from_rgb(0.75, 0.25, 0.25),
                Color::from_rgb(0.65, 0.2, 0.2),
                Color::from_rgb(0.5, 0.13, 0.13),
                Color::from_rgb(0.3, 0.05, 0.05),
            ),
            info: ColorScale::new(
                Color::from_rgb(0.95, 0.97, 1.0),
                Color::from_rgb(0.9, 0.94, 1.0),
                Color::from_rgb(0.8, 0.88, 1.0),
                Color::from_rgb(0.65, 0.78, 0.98),
                Color::from_rgb(0.45, 0.65, 0.9),
                Color::from_rgb(0.25, 0.5, 0.8),
                Color::from_rgb(0.2, 0.4, 0.7),
                Color::from_rgb(0.15, 0.3, 0.55),
                Color::from_rgb(0.08, 0.18, 0.35),
            ),
            background: BackgroundColors {
                base: Color::from_rgb(0.98, 0.98, 0.98),
                surface: Color::from_rgb(1.0, 1.0, 1.0),
                elevated: Color::from_rgb(0.96, 0.96, 0.96),
                overlay: Color::from_rgb(0.0, 0.0, 0.0).scale_alpha(0.3),
            },
            text: TextColors {
                primary: Color::from_rgb(0.1, 0.1, 0.1),
                secondary: Color::from_rgb(0.4, 0.4, 0.4),
                disabled: Color::from_rgb(0.6, 0.6, 0.6),
                inverse: Color::from_rgb(1.0, 1.0, 1.0),
                on_primary: Color::from_rgb(1.0, 1.0, 1.0),
                on_danger: Color::from_rgb(1.0, 1.0, 1.0),
                on_success: Color::from_rgb(1.0, 1.0, 1.0),
                on_warning: Color::from_rgb(0.1, 0.1, 0.1),
            },
            border: BorderColors {
                default: Color::from_rgb(0.85, 0.85, 0.85),
                emphasis: Color::from_rgb(0.65, 0.65, 0.65),
                strong: Color::from_rgb(0.4, 0.4, 0.4),
                focus: Color::from_rgb(0.2, 0.4, 0.6),
            },
            focus: FocusColors {
                primary: Color::from_rgb(0.2, 0.4, 0.6),
                secondary: Color::from_rgb(0.2, 0.6, 0.8),
                outline_width: 2.0,
                ring_width: 3.0,
            },
            status: StatusColors {
                success_text: Color::from_rgb(0.05, 0.45, 0.15),
                warning_text: Color::from_rgb(0.65, 0.38, 0.13),
                danger_text: Color::from_rgb(0.65, 0.2, 0.2),
                info_text: Color::from_rgb(0.2, 0.4, 0.7),
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: ColorScale::new(
                Color::from_rgb(0.15, 0.25, 0.4),
                Color::from_rgb(0.2, 0.32, 0.5),
                Color::from_rgb(0.28, 0.42, 0.65),
                Color::from_rgb(0.38, 0.55, 0.78),
                Color::from_rgb(0.5, 0.68, 0.88),
                Color::from_rgb(0.4, 0.6, 0.8),
                Color::from_rgb(0.55, 0.72, 0.9),
                Color::from_rgb(0.7, 0.85, 0.98),
                Color::from_rgb(0.85, 0.95, 1.0),
            ),
            secondary: ColorScale::new(
                Color::from_rgb(0.25, 0.25, 0.28),
                Color::from_rgb(0.32, 0.32, 0.35),
                Color::from_rgb(0.42, 0.42, 0.46),
                Color::from_rgb(0.55, 0.55, 0.6),
                Color::from_rgb(0.7, 0.7, 0.75),
                Color::from_rgb(0.82, 0.82, 0.88),
                Color::from_rgb(0.9, 0.9, 0.95),
                Color::from_rgb(0.95, 0.95, 0.98),
                Color::from_rgb(0.98, 0.98, 1.0),
            ),
            neutral: ColorScale::new(
                Color::from_rgb(0.12, 0.12, 0.15),
                Color::from_rgb(0.18, 0.18, 0.22),
                Color::from_rgb(0.24, 0.24, 0.28),
                Color::from_rgb(0.32, 0.32, 0.38),
                Color::from_rgb(0.42, 0.42, 0.5),
                Color::from_rgb(0.55, 0.55, 0.62),
                Color::from_rgb(0.7, 0.7, 0.78),
                Color::from_rgb(0.85, 0.85, 0.92),
                Color::from_rgb(0.95, 0.95, 0.98),
            ),
            success: ColorScale::new(
                Color::from_rgb(0.08, 0.2, 0.12),
                Color::from_rgb(0.1, 0.28, 0.18),
                Color::from_rgb(0.15, 0.38, 0.25),
                Color::from_rgb(0.22, 0.5, 0.35),
                Color::from_rgb(0.32, 0.65, 0.48),
                Color::from_rgb(0.3, 0.7, 0.4),
                Color::from_rgb(0.4, 0.8, 0.55),
                Color::from_rgb(0.55, 0.9, 0.7),
                Color::from_rgb(0.7, 0.98, 0.85),
            ),
            warning: ColorScale::new(
                Color::from_rgb(0.25, 0.18, 0.08),
                Color::from_rgb(0.35, 0.25, 0.1),
                Color::from_rgb(0.48, 0.33, 0.13),
                Color::from_rgb(0.62, 0.43, 0.17),
                Color::from_rgb(0.78, 0.55, 0.22),
                Color::from_rgb(0.9, 0.65, 0.3),
                Color::from_rgb(1.0, 0.78, 0.45),
                Color::from_rgb(1.0, 0.88, 0.6),
                Color::from_rgb(1.0, 0.98, 0.8),
            ),
            danger: ColorScale::new(
                Color::from_rgb(0.2, 0.08, 0.08),
                Color::from_rgb(0.3, 0.1, 0.1),
                Color::from_rgb(0.42, 0.15, 0.15),
                Color::from_rgb(0.58, 0.22, 0.22),
                Color::from_rgb(0.75, 0.32, 0.32),
                Color::from_rgb(0.9, 0.35, 0.35),
                Color::from_rgb(0.98, 0.5, 0.5),
                Color::from_rgb(1.0, 0.68, 0.68),
                Color::from_rgb(1.0, 0.85, 0.85),
            ),
            info: ColorScale::new(
                Color::from_rgb(0.1, 0.15, 0.25),
                Color::from_rgb(0.15, 0.22, 0.35),
                Color::from_rgb(0.2, 0.32, 0.48),
                Color::from_rgb(0.28, 0.42, 0.62),
                Color::from_rgb(0.38, 0.55, 0.78),
                Color::from_rgb(0.35, 0.55, 0.82),
                Color::from_rgb(0.48, 0.68, 0.9),
                Color::from_rgb(0.62, 0.82, 0.98),
                Color::from_rgb(0.8, 0.92, 1.0),
            ),
            background: BackgroundColors {
                base: Color::from_rgb(0.1, 0.1, 0.12),
                surface: Color::from_rgb(0.15, 0.15, 0.18),
                elevated: Color::from_rgb(0.2, 0.2, 0.24),
                overlay: Color::from_rgb(0.0, 0.0, 0.0).scale_alpha(0.5),
            },
            text: TextColors {
                primary: Color::from_rgb(0.92, 0.92, 0.95),
                secondary: Color::from_rgb(0.65, 0.65, 0.7),
                disabled: Color::from_rgb(0.45, 0.45, 0.5),
                inverse: Color::from_rgb(0.1, 0.1, 0.12),
                on_primary: Color::from_rgb(0.1, 0.1, 0.12),
                on_danger: Color::from_rgb(1.0, 1.0, 1.0),
                on_success: Color::from_rgb(1.0, 1.0, 1.0),
                on_warning: Color::from_rgb(0.1, 0.1, 0.12),
            },
            border: BorderColors {
                default: Color::from_rgb(0.3, 0.3, 0.35),
                emphasis: Color::from_rgb(0.45, 0.45, 0.5),
                strong: Color::from_rgb(0.6, 0.6, 0.65),
                focus: Color::from_rgb(0.4, 0.6, 0.8),
            },
            focus: FocusColors {
                primary: Color::from_rgb(0.4, 0.6, 0.8),
                secondary: Color::from_rgb(0.5, 0.75, 0.95),
                outline_width: 2.0,
                ring_width: 3.0,
            },
            status: StatusColors {
                success_text: Color::from_rgb(0.55, 0.9, 0.7),
                warning_text: Color::from_rgb(1.0, 0.88, 0.6),
                danger_text: Color::from_rgb(1.0, 0.68, 0.68),
                info_text: Color::from_rgb(0.62, 0.82, 0.98),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorScale {
    pub50: Color,
    pub100: Color,
    pub200: Color,
    pub300: Color,
    pub400: Color,
    pub500: Color,
    pub600: Color,
    pub700: Color,
    pub900: Color,
}

impl ColorScale {
    pub fn new(
        c50: Color,
        c100: Color,
        c200: Color,
        c300: Color,
        c400: Color,
        c500: Color,
        c600: Color,
        c700: Color,
        c900: Color,
    ) -> Self {
        Self {
            pub50: c50,
            pub100: c100,
            pub200: c200,
            pub300: c300,
            pub400: c400,
            pub500: c500,
            pub600: c600,
            pub700: c700,
            pub900: c900,
        }
    }

    pub fn scale(&self, level: u32) -> Color {
        match level {
            50 => self.pub50,
            100 => self.pub100,
            200 => self.pub200,
            300 => self.pub300,
            400 => self.pub400,
            500 => self.pub500,
            600 => self.pub600,
            700 => self.pub700,
            _ => self.pub900,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundColors {
    pub base: Color,
    pub surface: Color,
    pub elevated: Color,
    pub overlay: Color,
}

#[derive(Debug, Clone)]
pub struct TextColors {
    pub primary: Color,
    pub secondary: Color,
    pub disabled: Color,
    pub inverse: Color,
    pub on_primary: Color,
    pub on_danger: Color,
    pub on_success: Color,
    pub on_warning: Color,
}

#[derive(Debug, Clone)]
pub struct BorderColors {
    pub default: Color,
    pub emphasis: Color,
    pub strong: Color,
    pub focus: Color,
}

#[derive(Debug, Clone)]
pub struct FocusColors {
    pub primary: Color,
    pub secondary: Color,
    pub outline_width: f32,
    pub ring_width: f32,
}

#[derive(Debug, Clone)]
pub struct StatusColors {
    pub success_text: Color,
    pub warning_text: Color,
    pub danger_text: Color,
    pub info_text: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryColor {
    Blue,
    Purple,
    Teal,
    Orange,
    Pink,
    Gray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Success,
    Warning,
    Danger,
    Info,
    Neutral,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CefrBadgeStyle {
    pub bg_color: Color,
    pub text_color: Color,
    pub border_color: Color,
}

impl CefrBadgeStyle {
    pub fn for_level(level: crate::models::CefrLevel) -> Self {
        let colors = SemanticColors::light();
        match level {
            crate::models::CefrLevel::A1 | crate::models::CefrLevel::A2 => Self {
                bg_color: colors.success.scale(100),
                text_color: colors.success.scale(700),
                border_color: colors.success.scale(300),
            },
            crate::models::CefrLevel::B1 | crate::models::CefrLevel::B2 => Self {
                bg_color: colors.warning.scale(100),
                text_color: colors.warning.scale(700),
                border_color: colors.warning.scale(300),
            },
            crate::models::CefrLevel::C1 | crate::models::CefrLevel::C2 => Self {
                bg_color: colors.danger.scale(100),
                text_color: colors.danger.scale(700),
                border_color: colors.danger.scale(300),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpacingScale {
    pub unit: f32,
    pub scale: [f32; 9],
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            unit: 4.0,
            scale: [2.0, 4.0, 6.0, 8.0, 12.0, 16.0, 24.0, 32.0, 48.0],
        }
    }
}

impl SpacingScale {
    pub fn xxs(&self) -> f32 {
        self.scale[0]
    }
    pub fn xs(&self) -> f32 {
        self.scale[1]
    }
    pub fn sm(&self) -> f32 {
        self.scale[2]
    }
    pub fn md(&self) -> f32 {
        self.scale[3]
    }
    pub fn lg(&self) -> f32 {
        self.scale[4]
    }
    pub fn xl(&self) -> f32 {
        self.scale[5]
    }
    pub fn xxl(&self) -> f32 {
        self.scale[6]
    }
    pub fn xxxl(&self) -> f32 {
        self.scale[7]
    }
    pub fn huge(&self) -> f32 {
        self.scale[8]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpacingValue {
    Xxs,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
    Huge,
}

impl SpacingValue {
    pub fn to_px(&self, scale: &SpacingScale) -> f32 {
        match self {
            SpacingValue::Xxs => scale.xxs(),
            SpacingValue::Xs => scale.xs(),
            SpacingValue::Sm => scale.sm(),
            SpacingValue::Md => scale.md(),
            SpacingValue::Lg => scale.lg(),
            SpacingValue::Xl => scale.xl(),
            SpacingValue::Xxl => scale.xxl(),
            SpacingValue::Xxxl => scale.xxxl(),
            SpacingValue::Huge => scale.huge(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypographyScale {
    pub font_family: FontFamily,
    pub scale_factor: f32,
    pub sizes: TypographySizes,
    pub weights: FontWeights,
    pub line_heights: LineHeights,
}

impl Default for TypographyScale {
    fn default() -> Self {
        Self {
            font_family: FontFamily::System,
            scale_factor: 1.25,
            sizes: TypographySizes::default(),
            weights: FontWeights::default(),
            line_heights: LineHeights::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFamily {
    System,
    SansSerif,
    Serif,
    Monospace,
}

impl FontFamily {
    pub fn as_str(&self) -> &str {
        match self {
            FontFamily::System => {
                "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif"
            }
            FontFamily::SansSerif => "'Inter', 'SF Pro Display', -apple-system, sans-serif",
            FontFamily::Serif => "'Georgia', 'Times New Roman', serif",
            FontFamily::Monospace => "'SF Mono', 'Fira Code', 'Consolas', monospace",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FontWeights {
    pub regular: u16,
    pub medium: u16,
    pub semibold: u16,
    pub bold: u16,
}

impl Default for FontWeights {
    fn default() -> Self {
        Self {
            regular: 400,
            medium: 500,
            semibold: 600,
            bold: 700,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineHeights {
    pub tight: f32,
    pub normal: f32,
    pub relaxed: f32,
    pub loose: f32,
}

impl Default for LineHeights {
    fn default() -> Self {
        Self {
            tight: 1.2,
            normal: 1.5,
            relaxed: 1.75,
            loose: 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypographySizes {
    pub caption: f32,
    pub footnote: f32,
    pub body: f32,
    pub subtitle: f32,
    pub title: f32,
    pub heading: f32,
    pub display: f32,
}

impl Default for TypographySizes {
    fn default() -> Self {
        Self {
            caption: 11.0,
            footnote: 12.0,
            body: 14.0,
            subtitle: 16.0,
            title: 18.0,
            heading: 20.0,
            display: 24.0,
        }
    }
}

impl TypographySizes {
    pub fn size(&self, variant: TypographyVariant) -> f32 {
        match variant {
            TypographyVariant::Caption => self.caption,
            TypographyVariant::Footnote => self.footnote,
            TypographyVariant::Body => self.body,
            TypographyVariant::Subtitle => self.subtitle,
            TypographyVariant::Title => self.title,
            TypographyVariant::Heading => self.heading,
            TypographyVariant::Display => self.display,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypographyVariant {
    Caption,
    Footnote,
    Body,
    Subtitle,
    Title,
    Heading,
    Display,
}

#[derive(Debug, Clone)]
pub struct DimensionTokens {
    pub border_radius: BorderRadiusValues,
    pub icon: IconSizeValues,
    pub touch_target: TouchTargetSize,
    pub input_height: InputHeightTokens,
}

impl Default for DimensionTokens {
    fn default() -> Self {
        Self {
            border_radius: BorderRadiusValues::default(),
            icon: IconSizeValues::default(),
            touch_target: TouchTargetSize::default(),
            input_height: InputHeightTokens::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BorderRadiusValues {
    pub none: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub full: f32,
}

impl Default for BorderRadiusValues {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 6.0,
            lg: 8.0,
            xl: 12.0,
            full: 9999.0,
        }
    }
}

impl BorderRadiusValues {
    pub fn radius(&self, variant: RadiusVariant) -> f32 {
        match variant {
            RadiusVariant::None => self.none,
            RadiusVariant::Sm => self.sm,
            RadiusVariant::Md => self.md,
            RadiusVariant::Lg => self.lg,
            RadiusVariant::Xl => self.xl,
            RadiusVariant::Full => self.full,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiusVariant {
    None,
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

#[derive(Debug, Clone)]
pub struct IconSizeValues {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

impl Default for IconSizeValues {
    fn default() -> Self {
        Self {
            xs: 12.0,
            sm: 16.0,
            md: 20.0,
            lg: 24.0,
            xl: 32.0,
        }
    }
}

impl IconSizeValues {
    pub fn size(&self, variant: IconSizeVariant) -> f32 {
        match variant {
            IconSizeVariant::Xs => self.xs,
            IconSizeVariant::Sm => self.sm,
            IconSizeVariant::Md => self.md,
            IconSizeVariant::Lg => self.lg,
            IconSizeVariant::Xl => self.xl,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSizeVariant {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Debug, Clone)]
pub struct TouchTargetSize {
    pub minimum: f32,
    pub recommended: f32,
    pub comfortable: f32,
}

impl Default for TouchTargetSize {
    fn default() -> Self {
        Self {
            minimum: 44.0,
            recommended: 48.0,
            comfortable: 56.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputHeightTokens {
    pub single_line: f32,
    pub multi_line: f32,
}

impl Default for InputHeightTokens {
    fn default() -> Self {
        Self {
            single_line: 40.0,
            multi_line: 80.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MotionTokens {
    pub duration: DurationScale,
    pub easing: EasingFunctions,
}

impl Default for MotionTokens {
    fn default() -> Self {
        Self {
            duration: DurationScale::default(),
            easing: EasingFunctions::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DurationScale {
    Instant,
    Fast,
    Base,
    Slow,
    Deliberate,
}

impl DurationScale {
    pub fn as_millis(&self) -> u32 {
        match self {
            DurationScale::Instant => 0,
            DurationScale::Fast => 100,
            DurationScale::Base => 200,
            DurationScale::Slow => 300,
            DurationScale::Deliberate => 500,
        }
    }
}

impl Default for DurationScale {
    fn default() -> Self {
        Self::Base
    }
}

#[derive(Debug, Clone)]
pub struct EasingFunctions {
    pub default: &'static str,
    pub ease_in: &'static str,
    pub ease_out: &'static str,
    pub ease_in_out: &'static str,
    pub bounce: &'static str,
}

impl Default for EasingFunctions {
    fn default() -> Self {
        Self {
            default: "cubic-bezier(0.4, 0.0, 0.2, 1.0)",
            ease_in: "cubic-bezier(0.4, 0.0, 1.0, 1.0)",
            ease_out: "cubic-bezier(0.0, 0.0, 0.2, 1.0)",
            ease_in_out: "cubic-bezier(0.4, 0.0, 0.2, 1.0)",
            bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)",
        }
    }
}
