use super::Widget;
use crate::surface::Primitive;
use crystal::{BoxSizing, EmptyLayout};
use helium_core::color::Color;

/// A [`Widget`] that fills up all the available space.  
/// Note that `Spacer`'s have no effect when the parent `widget` has
/// an intrinsic size of `Shrink`, because the parent will try to be
/// as small as possible, hence the spacer will have 0 size.
pub struct Spacer {
    id: String,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(),
        }
    }
}

impl Widget for Spacer {
    fn layout(&self) -> Box<dyn crystal::Layout> {
        let mut layout = EmptyLayout::new();
        layout.id = self.id.clone();
        layout.intrinsic_size.width = BoxSizing::Flex(1);
        layout.intrinsic_size.height = BoxSizing::Flex(1);

        Box::new(layout)
    }

    fn primitive(&self) -> Primitive {
        Primitive::Rect {
            id: self.id.clone(),
            corner_radius: 0, // TODO add corner radius
            color: Color::default(),
        }
    }
}
