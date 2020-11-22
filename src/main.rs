mod components;
// mod node;
mod func;
mod types;
mod widgets;

use crate::components::make_ui;

use crate::func::Function;
use crate::types::Type;
use druid::{im::Vector, AppLauncher, Data, Lens, LocalizedString, PlatformError, WindowDesc};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    root: Function,
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
    let bin = Type::Primitive("Bin".into());
    let bin2 = Type::from(vec![bin.clone(); 2]);

    let top = Function::new_const("TOP", bin.clone());
    // let bot = Function::new_const("BOT", bin.clone());
    let x = Function::new_var("x", bin.clone());
    let y = Function::new_var("y", bin.clone());

    let mut and = Function::from_input("AND", vec![x, top], bin.clone());
    let mut or = Function::from_input("OR", vec![y, and], bin.clone());
    let mut not = Function::from_input("NOT", vec![or], bin);
    let data = AppState { root: not };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}
