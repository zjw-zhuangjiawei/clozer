//! MDict (.mdx) format parser.
//!
//! MDict is a popular dictionary format with a simple text-based structure:
//!
//! ```text
//! word1
//! <html definition 1>
//!
//! word2
//! <html definition 2>
//! ```
//!
//! Entries are separated by blank lines. The definition may contain HTML.

pub mod line_iter;
pub mod parser;

pub use parser::MdxParser;
