//! Import module for external data formats.
//!
//! Supports importing vocabulary from various dictionary formats:
//! - MDict (.mdx) dictionaries
//! - JSON (legacy format)
//!
//! ## Architecture
//!
//! ```text
//! import/
//! ├── mod.rs           # Module entry, re-exports
//! ├── error.rs         # ImportError enum
//! ├── mdx/
//! │   ├── mod.rs       # MDX module entry
//! │   ├── parser.rs    # MdxParser implementation
//! │   └── line_iter.rs # Line iterator for streaming
//! ```

pub mod error;
pub mod mdx;
