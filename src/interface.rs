use druid::widget::prelude::*;
use druid::widget::{
    Button, Checkbox, CrossAxisAlignment, Flex, Label, MainAxisAlignment, SizedBox, Stepper,
    Switch, TextBox, WidgetExt,
};
use druid::{Color, Data, Lens, WidgetId};

#[derive(Clone, Data, Lens)]
pub struct State {
    pub interface: Interface,
}

#[derive(Clone, Data, Lens)]
pub struct Interface {
    pub input_text: String,
    pub enabled: bool,
}

/// builds a child Flex widget from some parameters.
struct Rebuilder {
    inner: Box<dyn Widget<State>>,
}

impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, _data: &State) {
        self.inner = build_widget();
    }
}

impl Widget<State> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &State, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &State, data: &State, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &State,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &State, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn build_widget() -> Box<dyn Widget<State>> {
    let flex = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Start);
    let mut flex = flex.with_child(
        TextBox::new()
            .with_placeholder("Sample text")
            .lens(Interface::input_text),
    );
    flex.add_default_spacer();

    flex.add_child(
        Button::new("Clear").on_click(|_ctx, data: &mut Interface, _env| {
            data.input_text.clear();
            data.enabled = false;
        }),
    );
    flex.add_default_spacer();

    flex.add_child(Label::new(|data: &Interface, _: &Env| {
        data.input_text.clone()
    }));
    flex.add_default_spacer();

    flex.add_child(Checkbox::new("Demo").lens(Interface::enabled));
    flex.add_default_spacer();
    flex.add_child(Switch::new().lens(Interface::enabled));
    flex.add_default_spacer();

    let mut flex = SizedBox::new(flex);

    let flex = flex
        .padding(8.0)
        .border(Color::grey(0.6), 2.0)
        .rounded(5.0)
        .lens(State::interface);

    flex.boxed()
}

pub fn make_ui() -> impl Widget<State> {
    Rebuilder::new()
}
