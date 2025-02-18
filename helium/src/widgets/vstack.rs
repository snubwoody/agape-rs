use crate::{impl_layout, impl_style, widgets::Widget, Color};
use crystal::{AxisAlignment, Layout, VerticalLayout};
use helium_core::{colors::TRANSPARENT, Rgba};
use helium_renderer::Rect;

/// A [`Widget`] that places it's children vertically. The `vstack!` macro
/// provides convienient initialization and is likely how you will be creating an
/// `VStack` most of the time.
///
/// # Example using `vstack!`
///
/// ```
/// use helium::{vstack,widgets::{Circle,Text}};
///
/// vstack!{
/// 	Circle::new(15),
/// 	Text::new("Hello world")
/// };
///
/// ```
///
/// You can also simply use the struct method for initialization if you choose to,
/// which is what `vstack!` expands to
///
/// ```
/// use helium::widgets::{Text,VStack};
///
/// VStack::new()
/// 	.add_child(Text::new("Hello"))
/// 	.add_child(Text::new("World"));
///
/// ```
/// 
/// # Scrolling
/// `VStack`'s are not scrollable by default, to enable scrolling use the `scrollable()`
/// method.
/// 
/// ```
/// use helium::{vstack,widgets::Text};
/// 
/// vstack!{
/// 	Text::new("Hello"),
/// 	Text::new("world!"),
/// }
/// .scrollable();
/// ```
///
pub struct VStack {
    id: String,
    children: Vec<Box<dyn Widget>>,
    color: Color<Rgba>,
    layout: VerticalLayout,
    corner_radius: u32,
	/// Whether the `VStack` should be scrollable, false by default
	scollable:bool
}

impl VStack {
    pub fn new() -> Self {
        VStack {
            id: nanoid::nanoid!(),
            color: TRANSPARENT,
            children: vec![],
            layout: VerticalLayout::new(),
            corner_radius: 0,
			scollable: false
        }
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
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

	/// Enable scrolling
	pub fn scrollable(mut self) -> Self{
		self.scollable = true;
		self
	}

    pub fn align_center(mut self) -> Self {
        self.layout.main_axis_alignment = AxisAlignment::Center;
        self.layout.cross_axis_alignment = AxisAlignment::Center;
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

    impl_layout!();
    impl_style!();
}

impl Widget for VStack {
    fn id(&self) -> &str {
        &self.id
    }

	fn scroll(&mut self,delta:crystal::Position) {
		if !self.scollable {return}

		// The delta is usally really small values so we must scale it
		let scroll_speed = 5.0;
		// TODO change the scroll speeed based on whether it's a mouse pad or touch pad
		self.layout.scroll(delta.y * scroll_speed);
	}

    fn layout(&self, renderer: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let children_layout: Vec<Box<dyn Layout>> = self
            .children
            .iter()
            .map(|widget| widget.layout(renderer))
            .collect();

		
        let VerticalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
			scroll_offset,
            ..
        } = self.layout;

        // TODO use builder pattern?
        let layout = VerticalLayout {
            id: self.id.clone(),
            spacing,
            padding,
            intrinsic_size,
            cross_axis_alignment,
            main_axis_alignment,
            constraints,
			scroll_offset,
            children: children_layout,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn draw(&self, layout: &dyn crystal::Layout, renderer: &mut helium_renderer::Renderer) {
		let primitive = Rect::from(layout)
			.color(self.color.clone())
			.corner_radius(self.corner_radius as f32);
        
		renderer.draw([primitive]);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children
            .iter()
            .map(|child| child.as_ref())
            .collect::<Vec<_>>()
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        self.children.as_mut_slice()
    }
}

/// Creates an [`VStack`].  
///
/// `vstack!` allows [`VStack`]'s to be declared in a more declarative manner.  
///
/// ```
/// use helium::{vstack,widgets::{Button,Text}};
///
/// vstack!{
/// 	Text::new("Hello"),
/// 	Text::new("world")
/// };
/// ```
/// 
/// # Scrolling
/// `VStack`'s are not scrollable by default, to enable scrolling use the `scrollable()`
/// method.
/// 
/// ```
/// use helium::{vstack,widgets::Text};
/// 
/// vstack!{
/// 	Text::new("Hello"),
/// 	Text::new("world!"),
/// }
/// .scrollable();
/// ```
///
#[macro_export]
macro_rules! vstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::VStack::new()
			$(.add_child($child))*
		}

	};

	()=>{
		$crate::widgets::VStack::new()
	};
}
