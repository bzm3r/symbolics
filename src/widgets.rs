use std::cmp::Ordering;
use std::iter::Iterator;
use std::sync::Arc;

use druid::kurbo::{Point, Rect, Size};

use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    UpdateCtx, Widget, WidgetPod,
};

use crate::components::node;
use crate::func::Function;

/// A list widget for a variable-size collection of items.
pub struct FunctionList {
    children: Vec<WidgetPod<Function, Box<dyn Widget<Function>>>>,
}

impl FunctionList {
    /// Create a new list widget.
    pub fn new() -> Self {
        FunctionList {
            children: Vec::new(),
        }
    }

    /// When the widget is created or the data changes, create or remove children as needed
    ///
    /// Returns `true` if children were added or removed.
    fn update_child_count(&mut self, data: &Function, _env: &Env) -> bool {
        let len = self.children.len();
        match len.cmp(&data.data_len()) {
            Ordering::Greater => self.children.truncate(data.data_len()),
            Ordering::Less => data.iter().enumerate().for_each(|(i, _)| {
                if i >= len {
                    let child = WidgetPod::new(node());
                    self.children.push(child.boxed());
                }
            }),
            Ordering::Equal => (),
        }
        len != data.data_len()
    }
}

impl Widget<Function> for FunctionList {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Function, env: &Env) {
        // let mut children = self.children.iter_mut();
        // data.for_each_mut(|child_data, _| {
        //     if let Some(child) = children.next() {
        //         child.event(ctx, event, child_data, env);
        //     }
        // });
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Function, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            if self.update_child_count(data, env) {
                ctx.children_changed();
            }
        }

        let mut children = self.children.iter_mut();
        data.iter().for_each(|child_data| {
            if let Some(child) = children.next() {
                child.lifecycle(ctx, event, child_data, env);
            }
        });
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Function, data: &Function, env: &Env) {
        // we send update to children first, before adding or removing children;
        // this way we avoid sending update to newly added children, at the cost
        // of potentially updating children that are going to be removed.
        let mut children = self.children.iter_mut();
        data.iter().for_each(|child_data| {
            if let Some(child) = children.next() {
                child.update(ctx, child_data, env);
            }
        });

        if self.update_child_count(data, env) {
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Function,
        env: &Env,
    ) -> Size {
        let mut width = bc.min().width;
        let mut y = 0.0;

        let mut indent_x = 0.0;

        let mut paint_rect = Rect::ZERO;
        let mut children = self.children.iter_mut();
        data.iter().for_each(|child_data| {
            let child = match children.next() {
                Some(child) => child,
                None => {
                    return;
                }
            };
            let child_bc = BoxConstraints::new(
                Size::new(bc.min().width, 0.0),
                Size::new(bc.max().width, std::f64::INFINITY),
            );
            let child_size = child.layout(ctx, &child_bc, child_data, env);
            // let rect = Rect::from_origin_size(Point::new(0.0, y), child_size);
            child.set_origin(ctx, child_data, env, Point::new(indent_x, y));
            paint_rect = paint_rect.union(child.paint_rect());
            width = width.max(child_size.width);
            y += child_size.height;
        });

        let my_size = bc.constrain(Size::new(width, y));

        // What is this insets stuff doing??
        let insets = paint_rect - Rect::ZERO.with_size(my_size);
        ctx.set_paint_insets(insets);

        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Function, env: &Env) {
        let mut children = self.children.iter_mut();
        data.iter().for_each(|child_data| {
            if let Some(child) = children.next() {
                child.paint(ctx, child_data, env);
            }
        });
    }
}
