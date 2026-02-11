use clozer::App;

fn main() {
    tracing_subscriber::fmt::init();

    let _ = iced::application(|| App::new().with_sample_data(), App::update, App::view).run();
}
