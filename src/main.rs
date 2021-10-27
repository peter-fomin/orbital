use druid::{AppLauncher, WindowDesc};

use orbital::data::AppState;
use orbital::view::build_ui;

fn main() {
    let main_window = WindowDesc::new(build_ui).title("Lambert solver");

    let initial_state = AppState::initial_earth().lambert_problem;

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
