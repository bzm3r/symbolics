use druid::widget::prelude::*;
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Flex, Label, MainAxisAlignment, SizedBox, Stepper,
    Switch, TextBox, WidgetExt, Container
};
use druid::{Color, Data, Lens, WidgetId};
use crate::AppState;
use crate::widgets::NodeList;

use crate::node::Node;

// fn node(date: Node) -> impl Widget<Node> {

//     let label = Label::new(data.root)

// }


pub fn node() -> impl Widget<Node> + 'static {
    let label = Label::dynamic(|data: &Node, env: &Env| data.root.clone()).padding(10.)
        .background(Color::BLACK).border(Color::grey(0.6), 2.)
        .rounded(3.).on_click(Node::click);
    label
}

pub fn make_ui() -> impl Widget<AppState> {
    let list = NodeList::new();
    let column = Flex::column().with_child(list.lens(AppState::root));
    column
}



