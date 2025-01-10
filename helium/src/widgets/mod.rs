mod button;
mod circle;
mod container;
mod hstack;
pub mod icon;
mod image;
mod rect;
mod spacer;
mod text;
mod vstack;
use crate::{
    app::AppState,
    surface::{rect::RectSurface, Primitive, Surface},
};
pub use button::*;
pub use circle::*;
pub use container::*;
use crystal::Layout;
pub use hstack::*;
pub use image::*;
pub use rect::*;
pub use spacer::*;
pub use text::*;
pub use vstack::*;

/// The trait that all widgets must implement. Each `widget` must implement the build function
/// which returns a [`WidgetBody`]. `widgetbodies` are objects that hold information about
/// the widget.
pub trait Widget: WidgetIterator + Send + Sync {
    // TODO add an iter please
    /// Build the [`Widget`] into a primitive [`WidgetBody`] for
    /// rendering.
    fn layout(&self) -> Box<dyn Layout>;

	fn primitive(&self) -> Primitive;

    /// Load data in the background
    fn update(&mut self) {}

	fn children(&self) -> Vec<&dyn Widget>{
		vec![]
	}
}

pub struct WidgetIter<'a>{
	stack: Vec<&'a dyn Widget>
}

impl<'a> Iterator for WidgetIter<'a> {
	type Item = &'a dyn Widget;

	fn next(&mut self) -> Option<Self::Item>{
		// The order of the iterator doesn't really matter in this
		// case, we just want to iterate over all the widgets
		if let Some(widget) = self.stack.pop(){
			self.stack.extend(widget.children());
			return Some(widget);
		}
		None
	}
}

pub trait WidgetIterator {
    fn iter(&self) -> WidgetIter<'_>;
}

impl<T: Widget> WidgetIterator for T {
    fn iter(&self) -> WidgetIter<'_> {
        WidgetIter{stack:vec![self]}
    }
}


// TODO remove this and replace with modifiers
/// Implement common styling attributes
#[macro_export]
macro_rules! impl_style {
    () => {
        /// Change the [`Color`] of a [`Widget`].
        pub fn color(mut self, color: crate::Color) -> Self {
            self.color = color;
            self
        }
    };
}

/// Implement common methods for widgets
#[macro_export]
macro_rules! impl_widget {
    () => {
        pub fn fill(mut self) -> Self {
            self.layout.intrinsic_size.width = crystal::BoxSizing::Flex(1);
            self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(1);
            self
        }

        pub fn flex(mut self, factor: u8) -> Self {
            self.layout.intrinsic_size.width = crystal::BoxSizing::Flex(factor);
            self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
            self
        }

        pub fn fit(mut self) -> Self {
            self.layout.intrinsic_size.width = crystal::BoxSizing::Shrink;
            self.layout.intrinsic_size.height = crystal::BoxSizing::Shrink;
            self
        }

        pub fn fill_width(mut self) -> Self {
            self.layout.intrinsic_size.width = crystal::BoxSizing::Flex(1);
            self
        }

        pub fn fill_height(mut self) -> Self {
            self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(1);
            self
        }

        pub fn fixed_width(mut self, width: f32) -> Self {
            self.layout.intrinsic_size.width = crystal::BoxSizing::Fixed(width);
            self
        }

        pub fn fixed_height(mut self, height: f32) -> Self {
            self.layout.intrinsic_size.height = crystal::BoxSizing::Fixed(height);
            self
        }

        pub fn fit_width(mut self) -> Self {
            self.layout.intrinsic_size.width = crystal::BoxSizing::Shrink;
            self
        }

        pub fn fit_height(mut self) -> Self {
            self.layout.intrinsic_size.height = crystal::BoxSizing::Shrink;
            self
        }

        pub fn flex_width(mut self, factor: u8) -> Self {
            self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
            self
        }

        pub fn flex_height(mut self, factor: u8) -> Self {
            self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
            self
        }
    };
}
