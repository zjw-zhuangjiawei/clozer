use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

/// Locale DTO for UI display language selection.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    strum::VariantArray,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum LocaleDto {
    #[default]
    EnUS,
    ZhCN,
    JaJP,
    KoKR,
}

impl LocaleDto {
    pub fn to_langid(self) -> LanguageIdentifier {
        match self {
            LocaleDto::EnUS => "en-US".parse().unwrap(),
            LocaleDto::ZhCN => "zh-CN".parse().unwrap(),
            LocaleDto::JaJP => "ja-JP".parse().unwrap(),
            LocaleDto::KoKR => "ko-KR".parse().unwrap(),
        }
    }

    pub fn from_langid(langid: &LanguageIdentifier) -> Option<Self> {
        match langid.to_string().as_str() {
            "en-US" | "en" => Some(LocaleDto::EnUS),
            "zh-CN" | "zh" => Some(LocaleDto::ZhCN),
            "ja-JP" | "ja" => Some(LocaleDto::JaJP),
            "ko-KR" | "ko" => Some(LocaleDto::KoKR),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            LocaleDto::EnUS => "English",
            LocaleDto::ZhCN => "\u{4E2D}\u{6587}", // 中文
            LocaleDto::JaJP => "\u{65E5}\u{672C}\u{8A9E}", // 日本語
            LocaleDto::KoKR => "\u{D55C}\u{AD6D}\u{C5B4}", // 한국어
        }
    }
}
