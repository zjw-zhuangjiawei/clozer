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

    // Initialize tracing with configured log level
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("clozer", app_config.log_level.into_tracing_level()),
        )
        .init();

    // Application startup
    tracing::info!("Clozer starting up");
    tracing::info!("Data directory: {:?}", app_config.data_dir);
    tracing::info!("Log level: {:?}", app_config.log_level);

    let _ = iced::daemon(move || App::new(app_config.clone()), App::update, App::view)
        .title(App::title)
        .subscription(App::subscription)
        .theme(App::theme)
        .run();
}
