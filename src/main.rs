use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

use orbital::Vector3D;
use orbital::lambert::LambertSolver;

fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title("Solving a Lambert problem")
        .window_size((400.0, 400.0));

    let mu = 3.986004e5;
    let r1 = Vector3D {
        x: 5000.0,
        y: 10000.0,
        z: 2100.0
    };
    let r2 = Vector3D {
        x: -14600.0,
        y: 2500.0,
        z: 7000.0
    };
    let ls = LambertSolver::new(r1, r2, 3600.0, mu);
    
    AppLauncher::with_window(main_window).launch(ls).expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<LambertSolver> {
    let text_input = LocalizedString::new("Input two radius vectors and time of flight below");
    let label = Label::new(text_input);

    let layout = Flex::column().with_child(label);
    layout
}
