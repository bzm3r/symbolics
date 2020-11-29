use crate::widgets::FunctionList;
use crate::AppState;
use druid::widget::{Either, prelude::*};
use druid::widget::{Flex, Label, WidgetExt};
use druid::Color;

use crate::func::Function;

pub fn function_context() -> impl Widget<Option<Function>> {
    let either = Either::new()
    let label = Label::dynamic(|data: &Function, env: &Env| format!("{}", data))
        .padding(10.)
        .background(Color::BLACK)
        .border(Color::grey(0.6), 2.)
        .rounded(3.);
    label
}

pub fn function() -> impl Widget<Function> {
    let label = Label::dynamic(|data: &Function, env: &Env| format!("{}", data))
        .padding(10.)
        .background(Color::BLACK)
        .border(Color::grey(0.6), 2.)
        .rounded(3.);
    label
}

pub fn make_ui() -> impl Widget<AppState> {
    let list = FunctionList::new();
    let fn_context = function_context();
    let left_column = Flex::column().with_child(list.lens(AppState::root));
    let right_column = Flex::column().with_child(fn_context.lens(AppState::root));
    let row = Flex::row().with_child(left_column).with_child(right_column);
    row.debug_paint_layout()
}
