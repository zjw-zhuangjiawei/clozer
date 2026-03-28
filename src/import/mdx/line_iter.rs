//! Line iterator for streaming MDX file processing.
//!
//! Provides an iterator interface to read lines from an MDict file,
//! with support for blank-line delimited entries.

use std::io::{BufRead, BufReader};
use std::path::Path;

/// Maximum number of lines to buffer in memory per entry.
const LINE_BUFFER_SIZE: usize = 1000;

/// Iterator state for MDX entry parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// Looking for a word line
    Word,
    /// Reading definition lines
    Definition,
}

/// Represents a parsed MDX entry.
#[derive(Debug, Clone)]
pub struct MdxEntry {
    /// The headword
    pub word: String,
    /// The HTML definition (may be empty)
    pub definition: String,
}

/// Iterator over MDX entries, yielding one entry at a time.
#[derive(Debug)]
pub struct MdxLineIter<R> {
    reader: R,
    state: State,
    current_word: Option<String>,
    current_definition: String,
    line_number: usize,
}

impl<R: BufRead> MdxLineIter<R> {
    /// Create a new iterator from a buffered reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            state: State::Word,
            current_word: None,
            current_definition: String::new(),
            line_number: 0,
        }
    }

    /// Get the current line number (1-based).
    pub fn line_number(&self) -> usize {
        self.line_number
    }
}

impl MdxLineIter<BufReader<std::fs::File>> {
    /// Create from a file path.
    pub fn from_path(path: &Path) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Self::new(reader))
    }
}

/// Inner helper to process buffered lines into an entry.
fn buffer_to_entry(word: String, definition: String) -> Option<MdxEntry> {
    let trimmed_word = word.trim().to_string();
    if trimmed_word.is_empty() {
        return None;
    }
    Some(MdxEntry {
        word: trimmed_word,
        definition,
    })
}

// TODO: Implement the iterator logic
//
// The iterator should:
// 1. Read lines from the underlying reader
// 2. Accumulate definition lines until a blank line is found
// 3. Yield MdxEntry structs containing word + definition
// 4. Handle malformed entries gracefully

impl<R: BufRead> Iterator for MdxLineIter<R> {
    type Item = Result<MdxEntry, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement entry parsing logic
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_entry() {
        // TODO: Add tests for edge cases
    }
}
