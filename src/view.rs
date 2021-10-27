use druid::{
    widget::{Label, TextBox, Flex, Button},
    Widget, WidgetExt, Lens, LensExt, Env,
};

use super::data::{FloatFormatter};
use super::lambert::LambertSolver;
use super::vectors::Vector3D;

pub fn build_ui() -> impl Widget<LambertSolver> {
    let solve_button = Button::new("Solve")
    .on_click(|_ctx, ls: &mut LambertSolver, _env| ls.recalculate_solution());
    

    Flex::column()
        .with_child(
            Flex::column()
                .cross_axis_alignment(druid::widget::CrossAxisAlignment::End)
                .with_child(r_input_scope("r1 (km)", &LambertSolver::r1_v))
                .with_default_spacer()
                .with_child(r_input_scope("r2 (km)", &LambertSolver::r2_v))
                .with_default_spacer()
                .with_child(float_value_input_scope("time-of-flight (s)", LambertSolver::t))
                .with_default_spacer()
                .with_child(float_value_input_scope("gravitational parameter (km^3 / s)", LambertSolver::mu))
        )
        .with_default_spacer()
        .with_child(solve_button)
        .with_default_spacer()
        .with_child(converged_label())
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
    
    Flex::row()
        .cross_axis_alignment(druid::widget::CrossAxisAlignment::Baseline)
        .with_child(label)
        .with_child(textbox)
        .lens(lens)
    
}

fn converged_label() -> impl Widget<LambertSolver> {
    let converged_label = Label::new(|ls: &LambertSolver, _env: &Env| format!("Problem solved: {}", ls.is_solved()));
    converged_label
}
