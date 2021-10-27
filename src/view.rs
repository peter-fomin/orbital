use druid::{
    widget::{Label, TextBox, Flex, Button},
    Widget, WidgetExt, Lens, LensExt, Env,
};

use super::data::{FloatFormatter};
use crate::LambertSolver;
use crate::Vector3D;

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
        .with_child(
            Flex::row()
                .with_child(solve_button)
            .with_default_spacer()
            .with_child(converged_label())
        )
        .with_default_spacer()
        .with_child(
            Flex::column()
            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start)
            .with_child(v1_output_scope())
            .with_default_spacer()
            .with_child(v2_output_scope())
        )
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
    let converged_label = Label::new(|ls: &LambertSolver, _env: &Env| format!("Status: {}", if ls.is_solved() { "Solved"} else { "Not solved"}));
    converged_label
}

fn v1_output_scope() -> impl Widget<LambertSolver> {
    Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
        .with_child(Label::new(|ls: &LambertSolver, _env: &Env| {
            let v1 = ls.get_v1();
            format!("v1 = ({:.4}, {:.4}, {:.4}),", v1.x, v1.y, v1.z)
        }))
        .with_default_spacer()
        .with_child(Label::new(|ls: &LambertSolver, _env: &Env| {
            let v1 = ls.get_v1();
            format!("|v1| = {:.4} km/s", v1.mag())
        }))
}

fn v2_output_scope() -> impl Widget<LambertSolver> {
    Flex::row()
        .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
        .with_child(Label::new(|ls: &LambertSolver, _env: &Env| {
            let v2 = ls.get_v2();
            format!("v2 = ({:.4}, {:.4}, {:.4}),", v2.x, v2.y, v2.z)
        }))
        .with_default_spacer()
        .with_child(Label::new(|ls: &LambertSolver, _env: &Env| {
            let v2 = ls.get_v2();
            format!("|v2| = {:.4} km/s", v2.mag())
        }))
}