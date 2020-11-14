mod error;
mod interface;
mod sequence;
mod theory;

use crate::interface::{make_ui, Interface, State};

use druid::{AppLauncher, LocalizedString, PlatformError, WindowDesc};

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_ui)
        .window_size((720., 600.00))
        .with_min_size((620., 265.00))
        .title(LocalizedString::new("Flex Container Options"));

    let interface = Interface {
        input_text: "hello".into(),
        enabled: false,
    };

    let data = State { interface };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)?;
    Ok(())
}
