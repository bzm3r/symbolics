mod components;
// mod node;
mod func;
mod types;
mod widgets;

use crate::components::make_ui;

use crate::func::Function;
use crate::types::Type;
use druid::{AppLauncher, Data, Lens, LocalizedString, PlatformError, WindowDesc, im::{Vector, vector}};
use uuid::Uuid;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    root: Function,
    #[data(same_fn = "PartialEq::eq")]
    selected: Option<Uuid>
    // pub interface: Interface,
}

struct SingleFunctionLens;



impl Lens<AppState, Option<Function>> for SingleFunctionLens {
    fn with<V, F: FnOnce(&Option<Function>) -> V>(&self, data: &AppState, f: F) -> V {
        match data.selected {
            Some(selected) => {
                let function_context = data.root.by_id(&selected);
                f(&function_context)
            }
            None => f(&None)
        }
    }

    fn with_mut<V, F: FnOnce(&mut Option<Function>) -> V>(&self, data: &mut AppState, f: F) -> V {
        todo!()
    }
}

// impl Lens<AppState, Account> for ActiveAccount {
//     fn with<V, F: FnOnce(&Account) -> V>(&self, data: &AppState, f: F) -> V {
//         let active_id = data.account_list.active_account_id.clone().unwrap();
//         let index = data.account_list.get_index_from_key(&active_id);
//         f(&data.account_list.accounts[index])
//     }

//     fn with_mut<V, F: FnOnce(&mut Account) -> V>(&self, data: &mut AppState, f: F) -> V {
//         let active_id = data.account_list.active_account_id.clone().unwrap();
//         let index = data.account_list.get_index_from_key(&active_id);
//         f(&mut data.account_list.accounts[index])
//     }
// }


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
    let bot = Function::new_const("BOT", bin.clone());
    let x = Function::new_var("x", bin.clone());
    let y = Function::new_var("y", bin.clone());
    //let y = Function::from_input("y", vec![bot, top.clone()], bin.clone());

    let mut and = Function::new_concrete("AND", vector![x, top], bin.clone());
    let mut or = Function::new_concrete("OR", vector![y, and], bin.clone());
    let mut not = Function::new_concrete("NOT", vector![or], bin);
    let data = AppState { root: not, selected: None, };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}
