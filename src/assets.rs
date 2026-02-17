//! Embedded assets for the application.
//!
//! Uses `include_dir` to embed assets into the binary at compile time,
//! enabling single-binary distribution without external asset files.

use include_dir::{Dir, include_dir};

/// Embedded assets directory.
static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

/// Returns the contents of an SVG icon by name.
///
/// # Parameters
///
/// - `name`: The filename of the SVG icon (e.g., "check_box_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
///
/// # Returns
///
/// The raw bytes of the SVG file, or `None` if not found.
#[must_use]
pub fn get_svg(name: &str) -> Option<&'static [u8]> {
    ASSETS
        .get_file(format!("icon/{name}"))
        .map(|f| f.contents())
}
