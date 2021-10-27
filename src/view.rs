use druid::{
    widget::{Label, Flex},
    Widget,
};
use druid::Widget;
use druid::widget::Label;

use super::data::{AppState, VectorItem};

pub fn build_ui() -> impl Widget<AppState> {
    Label::new("Hello")
}

fn create_vector_widget(name: &str) -> impl Widget<VectorItem> {
    let label = Label::raw().lens(VectorItem::name);
    let x_label = Label::new("x:");
    let y_label = Label::new("y:");
    let z_label = Label::new("z:");

    
}