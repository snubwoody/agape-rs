use crate::{impl_style, impl_widget, view::{RectView, View}, widgets::Widget, Color};
use crystal::{AxisAlignment, HorizontalLayout, Layout};
use helium_core::color::TRANSPARENT;

pub struct HStack {
    id: String,
    children: Vec<Box<dyn Widget>>,
    color: Color,
    layout: HorizontalLayout,
}

impl HStack {
    pub fn new() -> Self {
        HStack {
            id: nanoid::nanoid!(),
            color: TRANSPARENT,
            children: vec![],
            layout: HorizontalLayout::new(),
        }
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

	fn view(&self) -> Box<dyn crate::view::View> {
		Box::new(
			RectView::new(&self.id)
				.color(self.color)	
		)		
	}

    fn update(&mut self) {
        self.children.iter_mut().for_each(|child| child.update());
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
/// use helium::{hstack,widgets::{Button,Text}};
///
/// hstack!{
/// 	Button::new("Click me"),
/// 	Text::new("Hello world")
/// };
///
/// ```
#[macro_export] // TODO add vec-like syntax hstack[widget;10]
macro_rules! hstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::HStack::new()
			$(.add_child($child))*
		}
	};
}
