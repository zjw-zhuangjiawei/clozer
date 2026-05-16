//! Internationalization (i18n) module.
//!
//! Provides locale-aware UI translation with compile-time embedded
//! translation files and en-US fallback.

pub mod locale;
pub mod manager;
pub mod translations;

pub use locale::LocaleDto;
pub use manager::I18nManager;
