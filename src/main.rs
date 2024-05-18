use app::SalmonRun;
use iced::Theme;

mod app;
mod grid;
mod instruction;
mod level;

fn main() {
    iced::program("Salmon Run", app::SalmonRun::update, app::SalmonRun::view)
        .theme(|_| Theme::Nord)
        .subscription(app::SalmonRun::subscription)
        .run_with(|| {
            SalmonRun::new()
        })
        .unwrap();
}
