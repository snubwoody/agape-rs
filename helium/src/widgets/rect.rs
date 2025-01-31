use super::Widget;
use crate::Color;
use crystal::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use helium_core::color::WHITE;
use nanoid::nanoid;

// TODO add BoxStyle struct
/// A simple rectangle
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rect {
    id: String,
    intrinsic_size: crystal::IntrinsicSize,
    color: Color,
    corner_radius: u32,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        let intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        };

        Self {
            id: nanoid!(),
            color: WHITE,
            intrinsic_size,
            corner_radius: 0,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// This event fires when the mouse cursor is over a [`Widget`]
    ///
    /// # Example
    /// ```
    /// use helium::widgets::Rect;
    ///
    /// Rect::new(150.0,150.0)
    /// 	.on_hover(||println!("Hello world"));
    /// ```
    pub fn on_hover(self, f: impl FnMut() + 'static) -> Self {
        self
    }

    /// This event fires when the mouse clicks on a [`Widget`].
    ///
    /// # Example
    /// ```
    /// use helium::widgets::Rect;
    ///
    /// Rect::new(150.0,150.0)
    /// 	.on_click(||println!("Hello world"));
    /// ```
    pub fn on_click(self, f: impl FnMut() + 'static) -> Self {
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    // TODO replace with impl_layout!()
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

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = self.intrinsic_size;
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([
            helium_renderer::Rect::new(layout.size().width, layout.size().height)
                .position(layout.position().x, layout.position().y)
                .color(self.color),
        ]);
    }
}
