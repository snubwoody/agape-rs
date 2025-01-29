//! [`Widget`]'s describe what you want to present onto the screen. Helium tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod button;
mod circle;
mod container;
mod hstack;
mod image;
mod rect;
mod spacer;
mod text;
mod text_field;
mod vstack;
pub mod icon;
pub use button::*;
pub use circle::*;
pub use container::*;
use crystal::Layout;
use helium_core::position::Bounds;
pub use hstack::*;
pub use image::*;
pub use rect::*;
pub use spacer::*;
pub use text::*;
pub use text_field::*;
pub use vstack::*;
use helium_renderer::Renderer;
use winit::event::{ElementState, MouseButton, WindowEvent};

/// The trait that all widgets must implement.
pub trait Widget: WidgetIterator {
    /// Build the [`Widget`] into a primitive [`WidgetBody`] for
    /// rendering.
    fn layout(&self) -> Box<dyn Layout>;

    /// Get the `id` of the [`Widget`]
    fn id(&self) -> &str;

    /// Get a [`Widget`] from the widget tree by it's `id`
    fn get(&self, id: &str) -> Option<&dyn Widget> {
        for widget in self.iter() {
            if widget.id() == id {
                return Some(widget);
            }
        }
        None
    }

	fn process_key(&mut self,key:&winit::keyboard::Key){}
	
	fn click(&mut self){}
	
	/// Set the [`Widget`]'s focus state to false when the cursor clicks outside
	/// the widget's bounds.
	fn unfocus(&mut self){}

	/// Draw the [`Widget`] to the screen
    fn draw(&self,layout:&dyn Layout,renderer:&mut Renderer);

    // TODO maybe make a test macro to make sure all widgets
    // handle this right
    /// Get the direct children of the [`Widget`]
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

	fn children_mut(&mut self) -> &mut [Box<dyn Widget>]{
		&mut []
	}

}


impl dyn Widget{
	fn dispatch_click(
		&mut self,
		mouse_pos:helium_core::Position,
		state:&winit::event::ElementState,
		button:&winit::event::MouseButton
	){
		match button {
			MouseButton::Left => {
				match state {
					ElementState::Pressed => {
						self.click();
					},
					_ =>{}
				}
			},
			_ => {}
		}
	}

	pub(crate) fn dispatch_event(
		&mut self,
		mouse_pos:helium_core::Position,
		layout_tree:&dyn Layout,
		window_event:&WindowEvent
	){
		// I feel like events might happen out of order because of winit's event loop but we shall find out
		// FIXME don't unwrap this pls
		let layout = layout_tree.get(self.id()).unwrap();
		match window_event {
			WindowEvent::KeyboardInput { event,.. } => {
				match event.state {
					ElementState::Pressed => {
						self.process_key(&event.logical_key);
					},
					ElementState::Released => {}
				}
			},
			WindowEvent::MouseInput { state, button,.. } => {
				let bounds = Bounds::new(layout.position(), layout.size());

				if bounds.within(&mouse_pos){
					self.dispatch_click(mouse_pos, state, button)
				}else {
					self.unfocus();
				}
			},
			_ => {}
		}

		for child in self.children_mut(){
			child.dispatch_event(mouse_pos,layout_tree,window_event);
		}
	}
}

// TODO test this
/// An iterator for the [`Widget`] tree.
pub struct WidgetIter<'a> {
    stack: Vec<&'a dyn Widget>,
}

impl<'a> Iterator for WidgetIter<'a> {
    type Item = &'a dyn Widget;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(widget) = self.stack.pop() {
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
        WidgetIter { stack: vec![self] }
    }
}

// TODO replace this with modifiers?
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
