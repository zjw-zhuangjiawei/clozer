//! MDict (.mdx) parser.
//!
//! This module provides the main interface for parsing MDict files
//! and converting them to Clozer's internal models.

use std::path::Path;

use crate::import::error::ImportError;
use crate::models::{Meaning, PartOfSpeech, Word};
use crate::registry::{MeaningRegistry, WordRegistry};

use super::line_iter::{MdxEntry, MdxLineIter};

/// Statistics from an import operation.
#[derive(Debug, Default)]
pub struct ImportStats {
    /// Number of words imported
    pub words: usize,
    /// Number of meanings imported
    pub meanings: usize,
    /// Number of entries skipped
    pub skipped: usize,
    /// Number of errors encountered
    pub errors: usize,
}

/// MDX parser for importing dictionary entries.
///
/// ## Usage
///
/// ```ignore
/// let mut parser = MdxParser::new();
/// let stats = parser.import_file("/path/to/dictionary.mdx", &mut word_reg, &mut meaning_reg)?;
/// println!("Imported {} words, {} meanings", stats.words, stats.meanings);
/// ```
#[derive(Debug, Default)]
pub struct MdxParser {
    /// Default part of speech for entries without POS info
    default_pos: PartOfSpeech,
    /// Maximum entries to import (0 = unlimited)
    entry_limit: usize,
}

impl MdxParser {
    /// Create a new parser with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the default part of speech for entries.
    pub fn with_default_pos(mut self, pos: PartOfSpeech) -> Self {
        self.default_pos = pos;
        self
    }

    /// Set an entry limit (0 = unlimited).
    pub fn with_entry_limit(mut self, limit: usize) -> Self {
        self.entry_limit = limit;
        self
    }

    /// Import entries from an MDX file.
    ///
    /// Returns statistics about the import operation.
    pub fn import_file(
        &mut self,
        path: &Path,
        word_registry: &mut WordRegistry,
        meaning_registry: &mut MeaningRegistry,
    ) -> Result<ImportStats, ImportError> {
        // TODO: Implement file import
        // 1. Open the file
        // 2. Create MdxLineIter
        // 3. Process entries
        // 4. Convert to Word + Meaning models
        // 5. Add to registries
        // 6. Return statistics
        todo!("Implement MDX file import")
    }

    /// Import entries from a reader (for testing or custom sources).
    pub fn import_reader<R: std::io::BufRead>(
        &mut self,
        reader: R,
        word_registry: &mut WordRegistry,
        meaning_registry: &mut MeaningRegistry,
    ) -> Result<ImportStats, ImportError> {
        // TODO: Implement reader-based import
        todo!("Implement MDX reader import")
    }

    /// Parse a single MDX entry into Word and Meaning models.
    ///
    /// Returns `None` if the entry should be skipped.
    fn parse_entry(&self, entry: &MdxEntry) -> Option<(Word, Meaning)> {
        // TODO: Implement entry parsing
        // 1. Create Word with the headword
        // 2. Create Meaning with definition and default POS
        // 3. Establish word -> meaning relationship
        // 4. Optionally detect POS from HTML
        // 5. Return (Word, Meaning) tuple
        todo!("Implement entry parsing")
    }

    /// Detect part of speech from HTML definition content.
    ///
    /// MDict dictionaries often include POS information in HTML tags
    /// or patterns like "n.", "v.", "adj." abbreviations.
    fn detect_pos(&self, _definition: &str) -> PartOfSpeech {
        // TODO: Implement POS detection from HTML
        // Common patterns:
        // - <abbr title="noun">n.</abbr>
        // - <font color...>n.</font>
        // - Abbreviations: n., v., adj., adv., etc.
        self.default_pos
    }

    /// Clean HTML from definition (optional, for plain text export).
    fn clean_html(&self, _definition: &str) -> String {
        // TODO: Implement HTML cleaning if needed
        // For now, we keep HTML as-is for rich display
        _definition.to_string()
    }
}

/// Convert a parsed entry to Word and Meaning models.
impl From<&MdxEntry> for (Word, Meaning) {
    fn from(entry: &MdxEntry) -> Self {
        // TODO: Implement proper conversion
        // This is a placeholder that won't compile - needs proper implementation
        todo!("Implement entry -> (Word, Meaning) conversion")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_simple_entry() {
        // TODO: Add tests for parsing
    }

    #[test]
    fn test_html_definition() {
        // TODO: Test that HTML definitions are preserved
    }

    #[test]
    fn test_multiline_definition() {
        // TODO: Test multi-line definitions
    }
}
