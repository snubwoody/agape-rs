use std::cell::RefCell;
use super::Widget;
use crate::{
    events::EventFn,
    view::RectView,
    Color,
};
use crystal::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use nanoid::nanoid;

/// A simple rectangle
pub struct Rect {
    id: String,
    intrinsic_size: crystal::IntrinsicSize,
    color: Color,
    corner_radius: u32,
    events: RefCell<Vec<EventFn>>,
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        let intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        };

        Self {
            id: nanoid!(),
            color,
            intrinsic_size,
            corner_radius: 0,
            events: RefCell::new(vec![]),
        }
    }

    pub fn on_hover(self, f: impl FnMut() + 'static) -> Self {
        let event = EventFn::OnHover(Box::new(f));
        self.events.borrow_mut().push(event);
        self
    }

    pub fn on_tap(self, f: impl FnMut() + 'static) -> Self {
        let event = EventFn::OnTap(Box::new(f));
        self.events.borrow_mut().push(event);
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    // TODO replace with impl_widget!()
    pub fn flex_width(mut self, factor: u8) -> Self {
        self.intrinsic_size.width = BoxSizing::Flex(factor);
        self
    }

    pub fn flex_height(mut self, factor: u8) -> Self {
        self.intrinsic_size.height = BoxSizing::Flex(factor);
        self
    }
}

impl Widget for Rect {
    fn id(&self) -> &str {
        &self.id
    }

	fn notify(&self,notification:&crate::events::Notify) {
		for event in self.events.borrow_mut().iter_mut(){
			event.run();
		}
	}

    fn layout(&self) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = self.intrinsic_size;
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn view(&self) -> Box<dyn crate::view::View> {
        Box::new(
            RectView::new(&self.id)
                .color(self.color)
                .corner_radius(self.corner_radius),
        )
    }
}
