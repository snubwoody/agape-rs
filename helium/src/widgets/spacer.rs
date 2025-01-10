use crystal::{BoxSizing, EmptyLayout};
use helium_core::color::Color;

use crate::surface::{rect::RectSurface, Primitive};

use super::{Widget, WidgetBody};

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
    fn build(&self) -> (super::WidgetBody, Box<dyn crystal::Layout>) {
        let body = WidgetBody::new().id(&self.id);

        let mut layout = EmptyLayout::new();
        layout.id = self.id.clone();
        layout.intrinsic_size.width = BoxSizing::Flex(1);
        layout.intrinsic_size.height = BoxSizing::Flex(1);

        (body, Box::new(layout))
    }

    fn surface(&self) -> Vec<Box<dyn crate::surface::Surface>> {
        vec![Box::new(RectSurface::new(&self.id))]
    }

	fn primitive(&self) -> Primitive {
		Primitive::Rect { 
			id: self.id.clone(), 
			corner_radius: 0, // TODO add corner radius 
			color:Color::default()
		}	
	}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{hstack, widgets::Rect};
    use crystal::{LayoutSolver, Size};
    use helium_core::color::BLACK;

    #[test]
    fn test_shrink_width() {
        let window = Size::new(500.0, 500.0);
        let widget = hstack! {
            Rect::new(20.0, 20.0, BLACK),
            Spacer::new(),
            Rect::new(20.0, 20.0, BLACK)
        }
        .fit_width();

        let (_, mut layout) = widget.build();

        LayoutSolver::solve(layout.as_mut(), window);
        dbg!(&layout);

        assert_eq!(layout.children()[1].size().width, 0.0);
    }

    #[test]
    fn test_width() {
        let window = Size::new(500.0, 500.0);
        let widget = hstack! {
            Rect::new(20.0, 20.0, BLACK),
            Spacer::new(),
            Rect::new(20.0, 20.0, BLACK)
        }
        .fill_width();

        let (_, mut layout) = widget.build();

        LayoutSolver::solve(layout.as_mut(), window);
        let spacer_width = window.width - 20.0 * 2.0;

        assert_eq!(layout.children()[1].size().width, spacer_width);
    }
}
