use clozer::{App, AppConfig, CliConfig, EnvConfig};

fn main() {
    tracing_subscriber::fmt::init();

    let _ = iced::application(
        || {
            // Load configuration (CLI > Env > Defaults)
            let cli = CliConfig::load(std::env::args_os());
            let env = EnvConfig::load().unwrap();
            let config = AppConfig::load(cli, env).expect("Failed to load config");
            App::new(config).with_sample_data()
        },
        App::update,
        App::view,
    )
    .title(App::title)
    .run();
}
