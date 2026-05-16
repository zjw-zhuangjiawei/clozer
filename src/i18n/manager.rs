use std::borrow::Cow;
use std::collections::HashMap;

use unic_langid::LanguageIdentifier;

use super::locale::LocaleDto;
use super::translations::TRANSLATIONS;

/// Manages UI translations with locale switching and fallback to en-US.
#[derive(Debug, Clone)]
pub struct I18nManager {
    /// All loaded translation bundles, keyed by locale.
    bundles: HashMap<LanguageIdentifier, HashMap<&'static str, &'static str>>,
    /// Currently active locale.
    current_locale: LanguageIdentifier,
}

impl I18nManager {
    /// Creates a new I18nManager with translations for all available locales.
    pub fn new() -> Self {
        let mut bundles: HashMap<LanguageIdentifier, HashMap<&'static str, &'static str>> =
            HashMap::new();

        for (lang_str, entries) in TRANSLATIONS {
            if let Ok(langid) = lang_str.parse::<LanguageIdentifier>() {
                let map: HashMap<&'static str, &'static str> =
                    entries.iter().map(|(k, v)| (*k, *v)).collect();
                bundles.insert(langid, map);
            }
        }

        Self {
            bundles,
            current_locale: LocaleDto::default().to_langid(),
        }
    }

    /// Looks up a translation key in the current locale, falling back to en-US.
    pub fn tr(&self, key: &str) -> Cow<'static, str> {
        // Try current locale
        if let Some(bundle) = self.bundles.get(&self.current_locale) {
            if let Some(value) = bundle.get(key) {
                return Cow::Borrowed(value);
            }
        }

        // Fallback to en-US
        if let Ok(en) = "en-US".parse::<LanguageIdentifier>() {
            if self.current_locale != en {
                if let Some(bundle) = self.bundles.get(&en) {
                    if let Some(value) = bundle.get(key) {
                        return Cow::Borrowed(value);
                    }
                }
            }
        }

        // Return the key itself if no translation found
        Cow::Owned(key.to_owned())
    }

    /// Looks up a translation key and replaces positional placeholders.
    /// Placeholders are `{0}`, `{1}`, etc. in the translation string.
    pub fn tr_with(&self, key: &str, args: &[&str]) -> String {
        let mut s = self.tr(key).to_string();
        for (i, arg) in args.iter().enumerate() {
            s = s.replace(&format!("{{{}}}", i), arg);
        }
        s
    }

    /// Sets the current locale.
    pub fn set_locale(&mut self, langid: LanguageIdentifier) {
        self.current_locale = langid;
    }

    /// Returns available locales that have translation bundles.
    pub fn available_locales(&self) -> Vec<LocaleDto> {
        self.bundles
            .keys()
            .filter_map(|langid| LocaleDto::from_langid(langid))
            .collect()
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}
