use clozer::App;

fn main() {
    tracing_subscriber::fmt::init();

    let _ = iced::daemon(
        || {
            let (app, task) = App::new(std::env::args_os(), std::env::vars());
            (app.with_sample_data(), task)
        },
        App::update,
        App::view,
    )
    .title(App::title)
    .subscription(App::subscription)
    .theme(App::theme)
    .run();
}
