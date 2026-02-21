use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use clozer::App;
use clozer::config::{AppConfig, CliConfig, EnvConfig};

fn main() {
    // Parse CLI args first to get log level for tracing init
    let cli = CliConfig::load(std::env::args_os());

    // Load env config
    let env = EnvConfig::load(std::env::vars()).unwrap_or_default();

    // Load app config with log level
    let app_config = AppConfig::load(cli, env).unwrap_or_default();

    // Initialize tracing with structured logging
    // Use target-based filtering and compact format for cleaner output
    let log_level = app_config.log_level.into_tracing_level();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("clozer", log_level)
                // Reduce noise from dependencies at info level
                .with_default(if log_level >= tracing::Level::DEBUG {
                    tracing::Level::WARN
                } else {
                    log_level
                }),
        )
        .init();

    // Application startup - use structured fields
    tracing::info!(
        target: "clozer::startup",
        data_dir = ?app_config.data_dir,
        log_level = ?app_config.log_level,
        "Clozer starting up"
    );

    let _ = clozer::App::run(app_config);
}
