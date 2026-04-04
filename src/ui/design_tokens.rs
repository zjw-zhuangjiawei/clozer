//! Centralized design tokens for consistent styling across the application.
//!
//! Design tokens are the atomic values that define the visual language:
//! - Typography (scale with ratios)
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

pub mod prelude {
    pub use super::{
        BorderRadiusValues, DimensionTokens, FontFamily, FontWeights, IconSizeValues,
        IconSizeVariant, InputHeightTokens, LineHeights, RadiusVariant, ScaleFactor, ScaleSystem,
        SpacingScale, SpacingValue, TouchTargetSize, TypographyScale, TypographySizes,
        TypographyVariant,
    };
}

#[derive(Debug, Clone)]
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
