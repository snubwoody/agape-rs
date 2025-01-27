use crate::{impl_style, impl_widget, widgets::Widget, Color};
use crystal::{AxisAlignment, HorizontalLayout, Layout};
use helium_core::color::TRANSPARENT;
use helium_renderer::Rect;

/// A [`Widget`] that places it's children horizontally. The `hstack!` macro
/// provides convienient initialization and is mostly how you be creating an `HStack`
/// most of the time.
///
/// # Example using `hstack!`
/// ```
/// use helium::{hstack,widgets::{Circle,Text}};
///
/// hstack!{
/// 	Circle::new(15),
/// 	Text::new("Hello world")
/// };
///
/// ```
///
/// You can also simply use the struct method for initialization if you choose to,
/// which is what `hstack!` expands to
///
/// ```
/// use helium::widgets::{Text,HStack};
///
/// HStack::new()
/// 	.add_child(Text::new("Hello"))
/// 	.add_child(Text::new("World"));
///
/// ```
///
pub struct HStack {
    id: String,
    children: Vec<Box<dyn Widget>>,
    color: Color,
	corner_radius:u32,
    layout: HorizontalLayout,
}

// TODO add get methods
impl HStack {
    pub fn new() -> Self {
        HStack {
            id: nanoid::nanoid!(),
            color: TRANSPARENT,
            children: vec![],
			corner_radius:0,
            layout: HorizontalLayout::new(),
        }
    }

	pub fn corner_radius(mut self,corner_radius:u32) -> Self{
		self.corner_radius = corner_radius;
		self
	}

    pub fn get(&self, index: usize) -> Option<&dyn Widget> {
        self.children.get(index).map(|w| &**w)
    }

    pub fn add_child(mut self, widget: impl Widget + 'static) -> Self {
        self.children.push(Box::new(widget));
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.layout.padding = padding;
        self
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.layout.spacing = spacing;
        self
    }

    pub fn main_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.main_axis_alignment = alignment;
        self
    }

    pub fn cross_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.cross_axis_alignment = alignment;
        self
    }

    impl_widget!();
    impl_style!();
}

// TODO test this
impl Widget for HStack {
    fn id(&self) -> &str {
        &self.id
    }

    fn tick(&mut self, elements: &[crate::events::Element]) {
        self.children
            .iter_mut()
            .for_each(|child| child.tick(elements));
    }

    fn layout(&self) -> Box<dyn Layout> {
        let children_layout: Vec<Box<dyn Layout>> =
            self.children.iter().map(|widget| widget.layout()).collect();

        let HorizontalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            ..
        } = self.layout;

        // TODO use builder pattern?
        let layout = HorizontalLayout {
            id: self.id.clone(),
            spacing,
            padding,
            intrinsic_size,
            cross_axis_alignment,
            main_axis_alignment,
            constraints,
            children: children_layout,
            ..Default::default()
        };

        Box::new(layout)
    }

	fn draw(&self,layout:&dyn crystal::Layout,renderer:&mut helium_renderer::Renderer) {
		renderer.draw([
			Rect::new(layout.size().width, layout.size().height)
				.position(layout.position().x, layout.position().y)
				.color(self.color)
				.corner_radius(self.corner_radius as f32)
		]);
	}

    fn children(&self) -> Vec<&dyn Widget> {
        self.children
            .iter()
            .map(|child| child.as_ref())
            .collect::<Vec<_>>()
    }
}

/// Creates an [`HStack`].  
/// `hstack!` allows [`HStack`]'s to be declared in a more declarative manner.
/// ```
/// use helium::{hstack,widgets::{Text}};
///
/// hstack!{
/// 	Text::new("Hello"),
/// 	Text::new("world"),
/// };
///
/// ```
// TODO add vec-like syntax hstack[widget;10]
#[macro_export]
macro_rules! hstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::HStack::new()
			$(.add_child($child))*
		}
	};
}
