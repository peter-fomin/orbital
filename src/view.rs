use druid::{
    widget::{Label, TextBox, Flex},
    Widget, WidgetExt, Lens, LensExt,
};

use super::data::{FloatFormatter};
use super::lambert::LambertSolver;
use super::vectors::Vector3D;

pub fn build_ui() -> impl Widget<LambertSolver> {
    Flex::column()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
        .with_child(r_input_scope("r1 (km)", &LambertSolver::r1_v))
        .with_child(r_input_scope("r2 (km)", &LambertSolver::r2_v))
        .with_child(float_value_input_scope("tof (s)", LambertSolver::t))
        .with_child(float_value_input_scope("mu (km^3 / s)", LambertSolver::mu))
        .center()
}

fn r_input_scope<L: Lens<LambertSolver, Vector3D> + 'static + Copy>(name: &str, lens: &L) -> impl Widget<LambertSolver> {
    

    Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
        .with_child(Label::new(name.to_string() + ":   "))
        .with_default_spacer()
        .with_child(float_value_input_scope("x", lens.then(Vector3D::x)))
        .with_child(float_value_input_scope("y", lens.then(Vector3D::y)))
        .with_child(float_value_input_scope("z", lens.then(Vector3D::z)))
    
}

fn float_value_input_scope<L: Lens<LambertSolver, f64>>(name: &str, lens: L) -> impl Widget<LambertSolver> {
    let label = Label::new(name.to_string() + ": ");

    let textbox = TextBox::new()
        .with_formatter(FloatFormatter)
        .validate_while_editing(false);
    
    print_type_of(&LambertSolver::mu);
    Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
        .with_child(label)
        .with_child(textbox)
        .lens(lens)
    
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}