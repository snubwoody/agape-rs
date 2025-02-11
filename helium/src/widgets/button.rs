use super::{Modifiers, Text, Widget};
use crate::impl_modifiers;
use crystal::{AxisAlignment, BlockLayout, Layout};
use helium_core::Color;

/// A `Button` is a [`Widget`] that wraps another [`Widget`] and responds to different
/// events such as `on_click` and `on_hover` events.
///
/// The simplest way to create a button is to use `Button::text()`
/// ```
/// use helium::widgets::Button;
///
/// let button = Button::text("Click me")
/// 	.on_click(||println!("Clicked!"));
/// ```
///
/// # Button wrapping a [`Widget`]
/// ```
/// use helium::{vstack,widgets::*};
///
/// let card = vstack!{
/// 	Text::new("Header"),
/// };
///
/// Button::new(card);
/// ```
pub struct Button<W> {
    id: String,
    pub color: Color,
    pub padding: u32,
    pub corner_radius: u32,
    child: W,
    modifiers: Modifiers,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Button<Text> {
    pub fn text(text: &str) -> Self {
        Self {
            id: nanoid::nanoid!(),
            color: Color::Hex("#615fff"),
            padding: 12,
            corner_radius: 0,
            child: Text::new(text),
            modifiers: Modifiers::new(),
            on_click: None,
        }
    }

    pub fn font_color(mut self, color: Color) -> Self {
        self.child.color = color;
        self
    }
}

impl<W: Widget> Button<W> {
    pub fn new(child: W) -> Self {
        Self {
            id: nanoid::nanoid!(),
            color: Color::Hex("#615fff"),
            padding: 12,
            corner_radius: 0,
            child,
            modifiers: Modifiers::new(),
            on_click: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn on_click(mut self, f: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    impl_modifiers!();
}

impl<W: Widget> Widget for Button<W> {
    fn id(&self) -> &str {
        &self.id
    }

    fn click(&mut self) {
        if let Some(func) = &mut self.on_click {
            func()
        }
    }

    fn layout(&self, renderer: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let mut layout = BlockLayout::new(self.child.layout(renderer));
        layout.intrinsic_size = self.modifiers.intrinsic_size;
        layout.padding = self.padding;
        layout.main_axis_alignment = AxisAlignment::Center; // TODO expose this
        layout.cross_axis_alignment = AxisAlignment::Center;
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([
            helium_renderer::Rect::new(layout.size().width, layout.size().height)
                .position(layout.position().x, layout.position().y)
                .color(self.color)
                .corner_radius(self.corner_radius as f32),
        ]);
    }
}
