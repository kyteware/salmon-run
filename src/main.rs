mod app;

fn main() {
    iced::run("test", app::SalmonRun::update, app::SalmonRun::view);
}
