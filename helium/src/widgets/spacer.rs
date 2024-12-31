use crystal::{BoxSizing, EmptyLayout};

use super::{Widget, WidgetBody};

/// A [`Widget`] that fills up all the available space.  
/// Note that `Spacer`'s have no effect when the parent `widget` has
/// an intrinsic size of `Shrink`, because the parent will try to be 
/// as small as possible, hence the spacer will have 0 size.
pub struct Spacer();

impl Widget for Spacer {
	fn build(&self) -> (super::WidgetBody,Box<dyn crystal::Layout>) {
		let body = WidgetBody::default();
		
		let mut layout = EmptyLayout::new();
		layout.id = body.id.clone();
		// TODO this might not work as intended if i make both sizes flex
		layout.intrinsic_size.width = BoxSizing::Flex(1);
		layout.intrinsic_size.height = BoxSizing::Flex(1);

		return (body,Box::new(layout))
	}
}

#[cfg(test)]
mod test{
    use super::*;
    use crystal::{LayoutSolver, Size};
    use helium_core::color::BLACK;
    use crate::{hstack, widgets::Rect};

	#[test]
	fn test_shrink_width(){
		let window = Size::new(500.0, 500.0);
		let widget = hstack!{
			Rect::new(20.0, 20.0, BLACK),
			Spacer(),
			Rect::new(20.0, 20.0, BLACK)
		}.fit_width();

		let (_,mut layout) = widget.build();

		LayoutSolver::solve(layout.as_mut(), window);
		dbg!(&layout);

		assert_eq!(layout.children()[1].size().width,0.0);
	}

	#[test]
	fn test_width(){
		let window = Size::new(500.0, 500.0);
		let widget = hstack!{
			Rect::new(20.0, 20.0, BLACK),
			Spacer(),
			Rect::new(20.0, 20.0, BLACK)
		}.fill_width();

		let (_,mut layout) = widget.build();

		LayoutSolver::solve(layout.as_mut(), window);
		let spacer_width = window.width - 20.0 * 2.0;

		assert_eq!(layout.children()[1].size().width,spacer_width);
	}
}