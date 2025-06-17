//! [`Widget`]'s describe what you want to present onto the screen. Helium tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod _await;
mod button;
mod circle;
mod container;
mod hstack;
pub mod icon;
mod image;
mod rect;
mod spacer;
mod text;
mod text_field;
mod vstack;

use resvg::tiny_skia;
use resvg::tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
pub use _await::*;
pub use button::*;
pub use circle::*;
pub use container::*;
use crystal::{BlockLayout, EmptyLayout, HorizontalLayout, Layout, VerticalLayout};
use helium_core::{Bounds, Color, Position, Rgba};
use helium_renderer::{Surface, Renderer};
pub use hstack::*;
pub use image::*;
pub use rect::*;
pub use spacer::*;
pub use text::*;
pub use text_field::*;
pub use vstack::*;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};

// TODO maybe have a build function that returns a layout and surface

pub trait Widget: WidgetIterator {
    fn render(&self,pixmap: &mut Pixmap){
        pixmap.fill(tiny_skia::Color::WHITE); let mut paint = Paint::default();
        paint.set_color(tiny_skia::Color::BLACK);
        let rect = tiny_skia::Rect::from_xywh(0.0,0.0,50.0,50.0).unwrap();
        let path = PathBuilder::from_rect(rect);
        pixmap.fill_path(&path,&paint,FillRule::Winding,Transform::identity(), None);
    }
    /// Get the widget's [`Layout`]
    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout>;
    
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

	/// Build the [`Widget`] into a [`WidgetBody`]
	fn build(&self,renderer: &mut Renderer) -> WidgetBody; 

    /// Runs every frame allowing [`Widget`]'s to manage any
    /// state they may have
    fn tick(&mut self) {}

    fn process_key(&mut self, key: &winit::keyboard::Key) {}

    fn click(&mut self) {}
    
	/// Respond to the user scrolling, triggered by either the touchpad or mousewheel.
	fn scroll(&mut self,delta:Position) {}

    /// Set the [`Widget`]'s focus state to false when the cursor clicks outside
    /// the widget's bounds.
    fn unfocus(&mut self) {}

    /// Draw the [`Widget`] to the screen
    fn draw(&self, layout: &dyn Layout, renderer: &mut Renderer);

    // TODO maybe make a test macro to make sure all widgets
    // handle this right
    /// Get the direct children of the [`Widget`]
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }
}

// TODO I've forgoten why this is seperate
impl dyn Widget {
    pub fn update(&mut self) {
        self.tick();
        for child in self.children_mut() {
            child.tick();
        }
    }

    /// Handles `winit`'s click event.
    fn dispatch_click(
        &mut self,
        state: &winit::event::ElementState,
        button: &winit::event::MouseButton,
    ) {
        match button {
            MouseButton::Left => match state {
                ElementState::Pressed => {
                    self.click();
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// Handles all `winit` events
    pub(crate) fn dispatch_event(
        &mut self,
        mouse_pos: helium_core::Position,
        layout_tree: &dyn Layout,
        window_event: &WindowEvent,
    ) {
        // I feel like events might happen out of order because of winit's event loop but we shall find out
        if let Some(layout) = layout_tree.get(self.id()) {
            match window_event {
                WindowEvent::KeyboardInput { event, .. } => match event.state {
                    ElementState::Pressed => {
                        self.process_key(&event.logical_key);
                    }
                    ElementState::Released => {}
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    let bounds = Bounds::new(layout.position(), layout.size());

                    if bounds.within(&mouse_pos) {
                        self.dispatch_click(state, button)
                    } else {
                        self.unfocus();
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let position = match delta {
                        MouseScrollDelta::LineDelta(x, y) => Position::new(*x, *y),
                        MouseScrollDelta::PixelDelta(pos) => {
                            Position::new(pos.x as f32, pos.y as f32)
                        }
                    };
					// TODO check for mouse position
					self.scroll(position);
                }
                _ => {}
            }
        } else {
            log::warn!("Widget: {} is missing a Layout", self.id())
        }

        for child in self.children_mut() {
            child.dispatch_event(mouse_pos, layout_tree, window_event);
        }
    }
}


#[derive(Debug,PartialEq, PartialOrd,Clone, Copy,Default)]
pub enum LayoutType{
	VerticalLayout,
	HorizontalLayout,
	BlockLayout,
	#[default]
	EmptyLayout,
}

#[derive(Debug,PartialEq, PartialOrd,Clone,Copy,Default)]
pub struct LayoutConfig{
	padding: u32,
	spacing: u32,
	scroll_offset: f32,
	intrinsic_size: crystal::IntrinsicSize,
	main_axis_alignment: crystal::AxisAlignment,
	cross_axis_alignment: crystal::AxisAlignment,
	_type: LayoutType
}

// TODO add empty, vertical, etc constructors
impl LayoutConfig{
	pub fn new() -> Self{
		Self::default()
	}

	pub fn block() -> Self{
		Self::default()
		.layout(LayoutType::BlockLayout)
	}

	pub fn empty() -> Self{
		Self::default()
		.layout(LayoutType::EmptyLayout)
	}

	pub fn horizontal() -> Self{
		Self::default()
		.layout(LayoutType::HorizontalLayout)
	}

	pub fn vertical() -> Self{
		Self::default()
		.layout(LayoutType::VerticalLayout)
	}

	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
		self
	}
	
	pub fn scroll_offset(mut self,scroll_offset:f32) -> Self{
		self.scroll_offset = scroll_offset;
		self
	}

	pub fn spacing(mut self,spacing:u32) -> Self{
		self.spacing = spacing;
		self
	}

	pub fn intrinsic_size(mut self, intrinsic_size: crystal::IntrinsicSize) -> Self{
		self.intrinsic_size = intrinsic_size;
		self
	}

	pub fn main_axis_alignment(mut self, main_axis_alignment:crystal::AxisAlignment) -> Self{
		self.main_axis_alignment = main_axis_alignment;
		self
	}

	pub fn cross_axis_alignment(mut self, cross_axis_alignment:crystal::AxisAlignment) -> Self{
		self.cross_axis_alignment = cross_axis_alignment;
		self
	}

	pub fn layout(mut self,_type: LayoutType) -> Self{
		self._type = _type;
		self
	}
}

// TODO size the Text
pub struct WidgetBody{
	id: String,
	layout: LayoutConfig,
	primitive: Surface,
	children: Vec<Box<WidgetBody>>
}

impl WidgetBody{
	/// Get the [`WidgetBody`]'s [`Surface`] by cloning and returning it.
	pub fn primitive(&self) -> Surface{
		self.primitive.clone()
	}

	/// Get the [`WidgetBody`]'s children
	pub fn children(&self) -> &[Box<WidgetBody>]{
		&self.children
	}

	/// Build the [`Widget`]'s layout from the [`LayoutConfig`]
	pub fn layout(&self) -> Box<dyn Layout>{
		let LayoutConfig { 
			padding, 
			spacing, 
			scroll_offset, 
			intrinsic_size, 
			main_axis_alignment, 
			cross_axis_alignment, 
			_type 
		} = self.layout;
		
		let layout: Box<dyn Layout> = match _type {
			LayoutType::BlockLayout => {
				let child = self.children[0].layout();	

				let mut layout = BlockLayout::new(child);
				layout.intrinsic_size = intrinsic_size;
				layout.padding = padding;
				layout.cross_axis_alignment = cross_axis_alignment;
				layout.main_axis_alignment = main_axis_alignment;

				Box::new(layout)
			},
			LayoutType::VerticalLayout => {
				let children:Vec<Box<dyn Layout>> = self.children
					.iter()
					.map(|child|child.layout())
					.collect();
				
				let layout = VerticalLayout{
					id: self.id.clone(),
					padding,
					spacing,
					scroll_offset,
					intrinsic_size,
					cross_axis_alignment,
					main_axis_alignment,
					children,
					..Default::default()
				};

				Box::new(layout)
			},
			LayoutType::EmptyLayout => {
				let layout = EmptyLayout{
					id: self.id.clone(),
					intrinsic_size,
					..Default::default()
				};

				Box::new(layout)
			},
			LayoutType::HorizontalLayout => {
				let children:Vec<Box<dyn Layout>> = self.children
					.iter()
					.map(|child|child.layout())
					.collect();
				
				let layout = HorizontalLayout{
					id: self.id.clone(),
					padding,
					spacing,
					intrinsic_size,
					cross_axis_alignment,
					main_axis_alignment,
					children,
					..Default::default()
				};

				Box::new(layout)
			},
		};

		layout
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

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Modifiers {
	padding: u32,
	spacing: u32,
	background_color: Color<Rgba>,
	foreground_color: Color<Rgba>,
    intrinsic_size: crystal::IntrinsicSize,
}

impl Modifiers {
    pub fn new() -> Self {
        Self::default()
    }
}

// TODO replace this with modifiers?
/// Implement common styling attributes
#[macro_export]
macro_rules! impl_style {
    () => {
        /// Change the [`Color`] of a [`Widget`].
        pub fn color(mut self, color: impl $crate::IntoColor<$crate::Rgba>) -> Self {
            self.color = color.into_color();
            self
        }
    };
}

/// Implement common methods for widgets
#[macro_export]
macro_rules! impl_modifiers {
    () => {
        pub fn fill(mut self) -> Self {
            self.modifiers.intrinsic_size.width = crystal::BoxSizing::Flex(1);
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Flex(1);
            self
        }

        pub fn flex(mut self, factor: u8) -> Self {
            self.modifiers.intrinsic_size.width = crystal::BoxSizing::Flex(factor);
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
            self
        }

        pub fn fit(mut self) -> Self {
            self.modifiers.intrinsic_size.width = crystal::BoxSizing::Shrink;
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Shrink;
            self
        }

        pub fn fill_width(mut self) -> Self {
            self.modifiers.intrinsic_size.width = crystal::BoxSizing::Flex(1);
            self
        }

        pub fn fill_height(mut self) -> Self {
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Flex(1);
            self
        }

        pub fn fixed_width(mut self, width: f32) -> Self {
            self.modifiers.intrinsic_size.width = crystal::BoxSizing::Fixed(width);
            self
        }

        pub fn fixed_height(mut self, height: f32) -> Self {
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Fixed(height);
            self
        }

        pub fn fit_width(mut self) -> Self {
            self.modifiers.intrinsic_size.width = crystal::BoxSizing::Shrink;
            self
        }

        pub fn fit_height(mut self) -> Self {
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Shrink;
            self
        }

        pub fn flex_width(mut self, factor: u8) -> Self {
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
            self
        }

        pub fn flex_height(mut self, factor: u8) -> Self {
            self.modifiers.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
            self
        }
    };
}

#[macro_export]
macro_rules! impl_layout {
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

#[cfg(test)]
mod tests{
    use crystal::{AxisAlignment, IntrinsicSize};
    use helium_renderer::IntoSurface;

    use super::*;

	// #[test]
	// fn build_layout_from_config(){
	// 	let config = LayoutConfig::vertical()
	// 		.padding(12)
	// 		.spacing(12)
	// 		.cross_axis_alignment(AxisAlignment::End)
	// 		.main_axis_alignment(AxisAlignment::End)
	// 		.intrinsic_size(IntrinsicSize::fill());

	// 	let primitive = helium_renderer::Rect::new(0.0, 0.0).into_surface();

	// 	let body = WidgetBody{
	// 		id: String::default(),
	// 		layout: config,
	// 		primitive,
	// 		children: vec![]
	// 	};
	// }
}
