use crate::widgets::FunctionList;
use crate::AppState;
use druid::widget::prelude::*;
use druid::widget::{Flex, Label, WidgetExt};
use druid::Color;

use crate::func::Function;

// fn node(date: Function) -> impl Widget<Function> {

//     let label = Label::new(data.root)

// }

pub fn node() -> impl Widget<Function> + 'static {
    let label = Label::dynamic(|data: &Function, env: &Env| data.label.clone())
        .padding(10.)
        .background(Color::BLACK)
        .border(Color::grey(0.6), 2.)
        .rounded(3.)
        .on_click(Function::click);
    label
}

pub fn make_ui() -> impl Widget<AppState> {
    let list = FunctionList::new();
    let column = Flex::column().with_child(list.lens(AppState::root));
    column
}
