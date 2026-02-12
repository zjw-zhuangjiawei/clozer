//! Application constants.

pub mod app {
    pub const NAME: &str = "clozer";
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    #[cfg(debug_assertions)]
    pub const DEBUG: bool = true;
    #[cfg(not(debug_assertions))]
    pub const DEBUG: bool = false;
}

pub mod paths {
    use std::path::PathBuf;

    pub const DATA: &str = "data";
    pub const CONFIG: &str = "clozer.toml";

    #[cfg(debug_assertions)]
    pub const DEBUG_DATA: &str = ".clozer-data";
    #[cfg(debug_assertions)]
    pub const DEBUG_CONFIG: &str = ".clozer.toml";

    /// Returns the default data directory.
    pub fn data_dir() -> PathBuf {
        if super::app::DEBUG {
            PathBuf::from(DEBUG_DATA)
        } else {
            dirs::data_dir()
                .map(|p| p.join(super::app::NAME))
                .unwrap_or_else(|| PathBuf::from(DATA))
        }
    }

    /// Returns the default config file path.
    pub fn config_file() -> PathBuf {
        if super::app::DEBUG {
            PathBuf::from(DEBUG_CONFIG)
        } else {
            dirs::config_dir()
                .map(|p| p.join(super::app::NAME).join(CONFIG))
                .unwrap_or_else(|| PathBuf::from(CONFIG))
        }
    }
}

pub mod db {
    pub const NAME: &str = "data.redb";
}
