//! Centralized design tokens for consistent styling across the application.
//!
//! Design tokens are the atomic values that define the visual language:
//! - Typography (font sizes, weights, line heights)
//! - Spacing (proportional system)
//! - Dimensions (components, borders, radii)
//!
//! # Design Principles
//!
//! 1. **Clarity First**: Visual hierarchy > information density
//! 2. **Consistency**: Same function = same style
//! 3. **Progressive Disclosure**: Core info first, details on demand
//! 4. **Accessibility**: WCAG 2.1 AA compliance required
//! 5. **Systematic**: Tokens drive everything, mathematical ratios control proportions

use std::fmt;

pub mod prelude {
    pub use super::{
        BorderRadiusValues, DimensionTokens, FontFamily, FontSize, IconSizeValues, IconSizeVariant,
        InputHeightTokens, RadiusVariant, ScaleFactor, ScaleSystem, Spacing, TouchTargetSize,
    };
}

#[derive(Debug, Clone)]
pub struct ScaleSystem {
    pub base_unit: f32,
    pub scale_factor: ScaleFactor,
}

#[derive(Debug, Clone, Copy)]
pub enum ScaleFactor {
    Small,
    Medium,
    Large,
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
    }
    pub fn sm(&self) -> f32 {
        self.size(-1)
    }
    pub fn md(&self) -> f32 {
        self.size(0)
    }
    pub fn lg(&self) -> f32 {
        self.size(1)
    }
    pub fn xl(&self) -> f32 {
        self.size(2)
    }
    pub fn xxl(&self) -> f32 {
        self.size(3)
    }
    pub fn xxxl(&self) -> f32 {
        self.size(4)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Spacing {
    pub xxs: f32,
    pub xs: f32,
    pub xs2: f32,
    pub s: f32,
    pub s2: f32,
    pub m: f32,
    pub l: f32,
    pub l2: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Spacing {
    pub const DEFAULT: Self = Self {
        xxs: 2.0,
        xs: 4.0,
        xs2: 5.0,
        s: 8.0,
        s2: 10.0,
        m: 12.0,
        l: 16.0,
        l2: 20.0,
        xl: 24.0,
        xxl: 32.0,
    };
}

impl fmt::Display for Spacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Spacing(xxs={}, xs={}, xs2={}, s={}, s2={}, m={}, l={}, l2={}, xl={}, xxl={})",
            self.xxs,
            self.xs,
            self.xs2,
            self.s,
            self.s2,
            self.m,
            self.l,
            self.l2,
            self.xl,
            self.xxl
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::Display, strum::VariantArray)]
pub enum FontSize {
    #[default]
    Caption,
    Footnote,
    Body,
    Subtitle,
    Title,
    Heading,
    Display,
}

impl FontSize {
    pub const fn px(self) -> f32 {
        match self {
            FontSize::Caption => 11.0,
            FontSize::Footnote => 12.0,
            FontSize::Body => 14.0,
            FontSize::Subtitle => 16.0,
            FontSize::Title => 18.0,
            FontSize::Heading => 20.0,
            FontSize::Display => 24.0,
        }
    }

    pub const fn line_height(self) -> f32 {
        match self {
            FontSize::Caption => 16.0,
            FontSize::Footnote => 18.0,
            FontSize::Body => 20.0,
            FontSize::Subtitle => 24.0,
            FontSize::Title => 26.0,
            FontSize::Heading => 28.0,
            FontSize::Display => 32.0,
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

#[derive(Debug, Clone, Default)]
pub struct DimensionTokens {
    pub border_radius: BorderRadiusValues,
    pub icon: IconSizeValues,
    pub touch_target: TouchTargetSize,
    pub input_height: InputHeightTokens,
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
