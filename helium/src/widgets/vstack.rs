use crate::{impl_style, impl_widget, view::RectView, widgets::Widget, Color};
use crystal::{AxisAlignment, Layout, VerticalLayout};
use helium_core::color::TRANSPARENT;

pub struct VStack {
    id: String,
    children: Vec<Box<dyn Widget>>,
    color: Color,
    layout: VerticalLayout,
    corner_radius: u32,
}

impl VStack {
    pub fn new() -> Self {
        VStack {
            id: nanoid::nanoid!(),
            color: TRANSPARENT,
            children: vec![],
            layout: VerticalLayout::new(),
            corner_radius: 0,
        }
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

	pub fn get(&self,index:usize) -> Option<&dyn Widget>{
		self.children.get(index).map(|w|&**w)
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

impl Widget for VStack {
    fn layout(&self) -> Box<dyn Layout> {
        let children_layout: Vec<Box<dyn Layout>> =
            self.children.iter().map(|widget| widget.layout()).collect();

        let VerticalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
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
            children: children_layout,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn view(&self) -> Box<dyn crate::view::View> {
        Box::new(
            RectView::new(&self.id)
                .color(self.color)
                .corner_radius(self.corner_radius),
        )
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children
            .iter()
            .map(|child| child.as_ref())
            .collect::<Vec<_>>()
    }

    fn update(&mut self) {
        self.children.iter_mut().for_each(|child| child.update());
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
/// 	Button::new("Click me"),
/// 	Text::new("Hello world")
/// };
/// ```
#[macro_export]
macro_rules! vstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::VStack::new()
			$(.add_child($child))*
		}

	};
}
