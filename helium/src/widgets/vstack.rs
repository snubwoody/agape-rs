use crate::{
    app::events::Event,
    impl_events, impl_style, impl_widget,
    surface::rect::RectSurface,
    widgets::{Widget, WidgetBody},
    Color,
};
use crystal::{AxisAlignment, Layout, VerticalLayout};
use helium_core::color::TRANSPARENT;

pub struct VStack {
    pub id: String,
    pub children: Vec<Box<dyn Widget>>,
    pub color: Color,
    pub layout: VerticalLayout,
}

impl VStack {
    pub fn new() -> Self {
        VStack {
            id: String::default(),
            color: TRANSPARENT,
            children: vec![],
            layout: VerticalLayout::new(),
        }
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
    impl_events!();
}

impl Widget for VStack {
    fn build(&self) -> (WidgetBody, Box<dyn Layout>) {
        let mut surface = RectSurface::default();
        surface.color(self.color.clone());

        let (children_body, children_layout): (Vec<Box<WidgetBody>>, Vec<Box<dyn Layout>>) = self
            .children
            .iter()
            .map(|widget| {
                let (body, layout) = widget.build();
                return (Box::new(body), layout);
            })
            .collect();

        let body = WidgetBody {
            id: self.id.clone(),
            children: children_body,
            surface: Box::new(surface),
            ..Default::default()
        };

        let VerticalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            ..
        } = self.layout;

        let layout = VerticalLayout {
            id: body.id.clone(),
            spacing,
            padding,
            intrinsic_size,
            constraints,
            main_axis_alignment,
            cross_axis_alignment,
            children: children_layout,
            ..Default::default()
        };

        (body, Box::new(layout))
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
			$crate::widgets::VStack{
				id:helium::nanoid!(),
				color:$crate::TRANSPARENT,
				layout:$crate::VerticalLayout::new(),
				children:vec![
					$(
						Box::new($child),
					)*
				]
			}
		}

	};
}
