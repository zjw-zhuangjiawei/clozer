//! Color system with OKLCH-based perceptual scales.
//!
//! This module provides a comprehensive color system featuring:
//! - **ColorScale**: Perceptual color scales generated from base colors using OKLCH
//! - **FunctionalPalette**: Semantic colors for success, warning, danger, info
//! - **ThemeColorPalette**: Complete palette combining all color tiers
//!
//! # OKLCH Color Space
//!
//! OKLCH (OKLab Lightness, Chroma, Hue) provides perceptually uniform color spacing,
//! meaning colors that are numerically distant appear equally distant to human perception.
//! This is crucial for creating accessible and consistent UI color systems.
//!
//! # Color Weights
//!
//! Color scales use a 50-900 weight system similar to Tailwind CSS:
//! - 50-100: Light tints
//! - 200-300: Light scales
//! - 400-500: Base scale
//! - 600-700: Dark scales
//! - 800-900: Dark shades

use super::semantic::SemanticPaletteBuilder;
pub use iced::Color;
use itertools::Itertools;
use palette::{FromColor, IntoColor, OklabHue, Oklch, Srgb};
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorWeight {
    W50,
    W100,
    W200,
    W300,
    W400,
    #[default]
    W500,
    W600,
    W700,
    W800,
    W900,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ColorScaleSpace {
    #[default]
    Oklch,
    Rgb,
}

pub trait IntoIcedColor: Sized {
    fn into_iced(self) -> Color;
}

impl IntoIcedColor for Srgb<f32> {
    fn into_iced(self) -> Color {
        Color::from_rgb(self.red, self.green, self.blue)
    }
}

impl IntoIcedColor for Oklch<f32> {
    fn into_iced(self) -> Color {
        Srgb::<f32>::from_color(self).into_iced()
    }
}

pub trait FromIcedColor: Sized {
    fn from_iced(c: Color) -> Self;
}

impl FromIcedColor for Srgb<f32> {
    fn from_iced(c: Color) -> Self {
        Srgb::new(c.r, c.g, c.b)
    }
}

impl FromIcedColor for Oklch<f32> {
    fn from_iced(c: Color) -> Self {
        Srgb::<f32>::from_iced(c).into_color()
    }
}

#[derive(Debug, Clone)]
pub struct ColorScale {
    colors: [Color; 10],
}

impl ColorScale {
    pub fn new(base: Color, mode: ColorMode, space: ColorScaleSpace) -> Self {
        let weights = [
            ColorWeight::W50,
            ColorWeight::W100,
            ColorWeight::W200,
            ColorWeight::W300,
            ColorWeight::W400,
            ColorWeight::W500,
            ColorWeight::W600,
            ColorWeight::W700,
            ColorWeight::W800,
            ColorWeight::W900,
        ];

        let colors = match space {
            ColorScaleSpace::Rgb => {
                let lightness: [f32; 10] = match mode {
                    ColorMode::Light => [
                        0.96, 0.92, 0.84, 0.70, 0.50, 0.13, 0.04, -0.08, -0.18, -0.28,
                    ],
                    ColorMode::Dark => [0.98, 0.94, 0.86, 0.76, 0.62, 0.48, 0.34, 0.22, 0.12, 0.05],
                };
                weights
                    .iter()
                    .zip(lightness.iter())
                    .map(|(_, &l)| {
                        let c = l.clamp(0.0, 1.0);
                        Color::from_rgb(c, c, c)
                    })
                    .collect_array()
                    .unwrap()
            }
            ColorScaleSpace::Oklch => {
                let base_srgb = Srgb::<f32>::from_iced(base);
                let base_oklch: Oklch<f32> = base_srgb.into_color();

                let deltas: [f32; 10] = match mode {
                    ColorMode::Light => [
                        0.95 - base_oklch.l,
                        0.90 - base_oklch.l,
                        0.80 - base_oklch.l,
                        0.65 - base_oklch.l,
                        0.45 - base_oklch.l,
                        0.0,
                        -0.12,
                        -0.22,
                        -0.32,
                        -0.42,
                    ],
                    ColorMode::Dark => [
                        0.97 - base_oklch.l,
                        0.93 - base_oklch.l,
                        0.85 - base_oklch.l,
                        0.75 - base_oklch.l,
                        0.60 - base_oklch.l,
                        0.0,
                        -0.10,
                        -0.18,
                        -0.26,
                        -0.34,
                    ],
                };

                let chroma_scale: [f32; 10] = match mode {
                    ColorMode::Light => [
                        0.02,
                        0.04,
                        0.07,
                        0.10,
                        0.12,
                        base_oklch.chroma,
                        base_oklch.chroma * 0.95,
                        base_oklch.chroma * 0.90,
                        base_oklch.chroma * 0.85,
                        base_oklch.chroma * 0.80,
                    ],
                    ColorMode::Dark => [
                        0.02,
                        0.04,
                        0.06,
                        0.09,
                        0.11,
                        base_oklch.chroma,
                        base_oklch.chroma * 1.10,
                        base_oklch.chroma * 1.15,
                        base_oklch.chroma * 1.20,
                        base_oklch.chroma * 1.25,
                    ],
                };

                let base_hue: f32 = base_oklch.hue.into_degrees();

                weights
                    .iter()
                    .zip(deltas.iter())
                    .zip(chroma_scale.iter())
                    .map(|((&weight, &delta), chroma)| {
                        let _weight = weight;
                        let new_l = (base_oklch.l + delta).clamp(0.0, 1.0);
                        let new_c = (*chroma).clamp(0.0, 0.4);
                        let oklch = Oklch::new(new_l, new_c, OklabHue::from_degrees(base_hue));
                        oklch.into_iced()
                    })
                    .collect_array()
                    .unwrap()
            }
        };

        Self { colors }
    }

    pub fn get(&self, weight: ColorWeight) -> Color {
        let idx = match weight {
            ColorWeight::W50 => 0,
            ColorWeight::W100 => 1,
            ColorWeight::W200 => 2,
            ColorWeight::W300 => 3,
            ColorWeight::W400 => 4,
            ColorWeight::W500 => 5,
            ColorWeight::W600 => 6,
            ColorWeight::W700 => 7,
            ColorWeight::W800 => 8,
            ColorWeight::W900 => 9,
        };
        self.colors[idx]
    }

    pub fn w50(&self) -> Color {
        self.get(ColorWeight::W50)
    }
    pub fn w100(&self) -> Color {
        self.get(ColorWeight::W100)
    }
    pub fn w200(&self) -> Color {
        self.get(ColorWeight::W200)
    }
    pub fn w300(&self) -> Color {
        self.get(ColorWeight::W300)
    }
    pub fn w400(&self) -> Color {
        self.get(ColorWeight::W400)
    }
    pub fn w500(&self) -> Color {
        self.get(ColorWeight::W500)
    }
    pub fn w600(&self) -> Color {
        self.get(ColorWeight::W600)
    }
    pub fn w700(&self) -> Color {
        self.get(ColorWeight::W700)
    }
    pub fn w800(&self) -> Color {
        self.get(ColorWeight::W800)
    }
    pub fn w900(&self) -> Color {
        self.get(ColorWeight::W900)
    }
}

impl Index<ColorWeight> for ColorScale {
    type Output = Color;

    fn index(&self, index: ColorWeight) -> &Self::Output {
        &self.colors[index as usize]
    }
}

#[derive(Debug, Clone)]
pub struct FunctionalPalette {
    pub success: ColorScale,
    pub warning: ColorScale,
    pub danger: ColorScale,
    pub info: ColorScale,
}

impl FunctionalPalette {
    pub fn new(mode: ColorMode) -> Self {
        let success_base = match mode {
            ColorMode::Light => Color::from_rgb(0.22, 0.62, 0.38),
            ColorMode::Dark => Color::from_rgb(0.25, 0.70, 0.45),
        };
        let warning_base = match mode {
            ColorMode::Light => Color::from_rgb(0.85, 0.55, 0.20),
            ColorMode::Dark => Color::from_rgb(0.88, 0.65, 0.30),
        };
        let danger_base = match mode {
            ColorMode::Light => Color::from_rgb(0.70, 0.25, 0.25),
            ColorMode::Dark => Color::from_rgb(0.88, 0.38, 0.38),
        };
        let info_base = match mode {
            ColorMode::Light => Color::from_rgb(0.30, 0.56, 0.57),
            ColorMode::Dark => Color::from_rgb(0.40, 0.60, 0.62),
        };

        Self {
            success: ColorScale::new(success_base, mode, ColorScaleSpace::Oklch),
            warning: ColorScale::new(warning_base, mode, ColorScaleSpace::Oklch),
            danger: ColorScale::new(danger_base, mode, ColorScaleSpace::Oklch),
            info: ColorScale::new(info_base, mode, ColorScaleSpace::Oklch),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub primary: ColorScale,
    pub neutral: ColorScale,
    pub functional: FunctionalPalette,
    pub accent: ColorScale,
    pub semantic: super::semantic::SemanticPalette,
}

impl ThemeColors {
    pub fn new(mode: ColorMode) -> Self {
        let primary_base = match mode {
            ColorMode::Light => Color::from_rgb(0.30, 0.56, 0.57),
            ColorMode::Dark => Color::from_rgb(0.40, 0.60, 0.62),
        };
        let accent_base = match mode {
            ColorMode::Light => Color::from_rgb(0.45, 0.45, 0.50),
            ColorMode::Dark => Color::from_rgb(0.62, 0.62, 0.68),
        };
        let neutral_base = match mode {
            ColorMode::Light => Color::from_rgb(0.13, 0.13, 0.13),
            ColorMode::Dark => Color::from_rgb(0.95, 0.95, 0.95),
        };

        let primary = ColorScale::new(primary_base, mode, ColorScaleSpace::Oklch);
        let neutral = ColorScale::new(neutral_base, mode, ColorScaleSpace::Rgb);
        let functional = FunctionalPalette::new(mode);
        let accent = ColorScale::new(accent_base, mode, ColorScaleSpace::Oklch);

        let semantic =
            SemanticPaletteBuilder::new(mode, primary.clone(), neutral.clone(), functional.clone())
                .build();

        Self {
            primary,
            neutral,
            functional,
            accent,
            semantic,
        }
    }

    pub fn light() -> Self {
        Self::new(ColorMode::Light)
    }

    pub fn dark() -> Self {
        Self::new(ColorMode::Dark)
    }
}
