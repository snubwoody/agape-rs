use super::*;
use crate::impl_modifiers;
use crystal::{AxisAlignment, BlockLayout, Layout};
use helium_core::{Color, IntoColor, Rgba};
use helium_renderer::IntoSurface;

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
    id: GlobalId,
    pub color: Color<Rgba>,
    pub padding: u32,
    pub corner_radius: u32,
    child: W,
    modifiers: Modifiers,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Button<Text> {
    pub fn text(text: &str) -> Self {
        Self {
            id: GlobalId::new(),
            color: Color::rgb(52, 68, 108),
            padding: 12,
            corner_radius: 0,
            child: Text::new(text),
            modifiers: Modifiers::new(),
            on_click: None,
        }
    }

    pub fn font_color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.child.color = color.into_color();
        self
    }
}

impl<W: Widget> Button<W> {
    pub fn new(child: W) -> Self {
        Self {
            id: GlobalId::new(),
            color: Color::rgb(0, 0, 0),
            padding: 12,
            corner_radius: 24,
            child,
            modifiers: Modifiers::new(),
            on_click: None,
        }
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
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
    fn id(&self) -> GlobalId {
        self.id
    }

    fn click(&mut self) {
        if let Some(func) = &mut self.on_click {
            func()
        }
    }

	fn build(&self,renderer: &mut helium_renderer::Renderer) -> WidgetBody{
		let mut layout = BlockLayout::new(self.child.layout(renderer));
        layout.intrinsic_size = self.modifiers.intrinsic_size;
        layout.padding = self.padding;
        layout.main_axis_alignment = AxisAlignment::Center; // TODO expose this
        layout.cross_axis_alignment = AxisAlignment::Center;
        layout.id = self.id.clone();

		let primitive = helium_renderer::RectSurface::new(0.0,0.0)
            .color(self.color.clone())
            .corner_radius(self.corner_radius as f32)
			.into_surface();
		let child = self.child.build(renderer);

		let layout = LayoutConfig::block()
			.padding(self.padding)
			.main_axis_alignment(AxisAlignment::Center)
			.cross_axis_alignment(AxisAlignment::Center)
			.intrinsic_size(self.modifiers.intrinsic_size);

		WidgetBody{
			id: self.id.clone(),
			primitive,
			layout,
			children: vec![Box::new(child)]
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
            helium_renderer::RectSurface::new(layout.size().width, layout.size().height)
                .position(layout.position().x, layout.position().y)
                .color(self.color.clone())
                .corner_radius(self.corner_radius as f32),
        ]);
    }
}
