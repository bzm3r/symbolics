mod error;
//mod interface;
mod components;
mod node;
mod theory;
mod widgets;

use crate::components::make_ui;
// use crate::interface::{make_ui, Interface, State};

use druid::{im::Vector, AppLauncher, Data, Lens, LocalizedString, PlatformError, WindowDesc};
use node::Node;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    root: Node,
    // pub interface: Interface,
}

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui)
        .window_size((720., 600.00))
        .with_min_size((620., 265.00))
        .title(LocalizedString::new("Symbolics"));

    // let interface = Interface {
    //     input_text: "hello".into(),
    //     enabled: false,
    // };

    // (x OR y) AND z
    let z = Node::new("z");
    let x = Node::new("x");
    let y = Node::new("y");
    let x_or_y = Node::new("OR").set_children(vec![x, y]);
    let root = Node::new("AND").set_children(vec![x_or_y, z]);

    let data = AppState { root };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}
