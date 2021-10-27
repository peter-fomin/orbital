use druid::{AppLauncher, WindowDesc};

use orbital::data::AppState;
use orbital::view::build_ui;

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Lambert solver");
    
    let initial_state = AppState::test().lambert_problem;

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}




// struct Container {
//     pub val: f64,
// }

// impl Formatter<f64> for Container {
//     fn format(&self, value: &f64) -> String {
//         format!("{}", value)
//     }

//     fn validate_partial_input(&self, input: &str, sel: &Selection) -> Validation {
//         Validation::success()
//     }

//     fn value(&self, input: &str) -> Result<f64, ValidationError> {
//         input.parse().map_err(ValidationError::new)
//     }
// }


// fn main() {
//     let main_window = WindowDesc::new(build_root_widget)
//         .title("Solving a Lambert problem")
//         .window_size((400.0, 400.0));

//     let mu = 3.986004e5;
//     let r1 = Vector3D {
//         x: 5000.0,
//         y: 10000.0,
//         z: 2100.0
//     };
//     let r2 = Vector3D {
//         x: -14600.0,
//         y: 2500.0,
//         z: 7000.0
//     };
//     let ls = LambertSolver::new(r1, r2, 3600.0, mu);
    
//     AppLauncher::with_window(main_window).launch(ls).expect("Failed to launch application");
// }

// fn build_root_widget() -> impl Widget<Vector3D> {
//     let label = Label::new("Input two radius vectors and time of flight below");


//     let layout = Flex::column().with_child(label).with_child(build_vector_input_string("r1"));
//     layout
// }

// fn build_r1_input_row() -> Flex<LambertSolver> {
//     let textbox_width: f64 = 60.0;

//     let label = Label::new(name.to_owned() + ":   ");
//     let x_label = Label::new("x:");
//     let y_label = Label::new("y:");
//     let z_label = Label::new("z:");

//     let x_field = TextBox::new().with_formatter(Container {val: 0.0});
//     let y_field = TextBox::new().with_formatter(Container {val: 0.0});
//     let z_field = TextBox::new().with_formatter(Container {val: 0.0});

//     Flex::row()
//         .with_child(label)
//         .with_child(x_label)
//         .with_child(x_field)
//         .with_child(y_label)
//         .with_child(y_field)
//         .with_child(z_label)
//         .with_child(z_field)
// }

// fn build_value_input(name: &str) -> Flex<Vector3D> {
//     let textbox_width: f64 = 60.0;

//     let label = Label::new(name.to_owned() + ":   ");
//     let x_label = Label::new("x:");
//     let y_label = Label::new("y:");
//     let z_label = Label::new("z:");

//     let x_field = TextBox::new().with_formatter(Container {val: 0.0});
//     let y_field = TextBox::new().with_formatter(Container {val: 0.0});
//     let z_field = TextBox::new().with_formatter(Container {val: 0.0});

//     Flex::row()
//         .with_child(label)
//         .with_child(x_label)
//         .with_child(x_field)
//         .with_child(y_label)
//         .with_child(y_field)
//         .with_child(z_label)
//         .with_child(z_field)
// }