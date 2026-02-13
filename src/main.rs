use clozer::App;

fn main() {
    tracing_subscriber::fmt::init();

    let _ = iced::application(
        || App::new(std::env::args_os(), std::env::vars()).with_sample_data(),
        App::update,
        App::view,
    )
    .title(App::title)
    .subscription(App::subscription_window_close)
    .run();
}
