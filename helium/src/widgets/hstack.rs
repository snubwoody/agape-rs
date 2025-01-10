use crate::{
    app::events::Event,
    impl_events, impl_style, impl_widget,
    surface::{rect::RectSurface,Primitive},
    widgets::{Widget, WidgetBody},
    Color,
};
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
    impl_events!();
}

// TODO test this
impl Widget for HStack {
    fn build(&self) -> (WidgetBody, Box<dyn Layout>) {
        let mut surface = RectSurface::new(&self.id);
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
            surface: Box::new(surface),
            children: children_body,
            ..Default::default()
        };

        let HorizontalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            ..
        } = self.layout;

        // TODO maybe impl into?
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

        (body, Box::new(layout))
    }

    fn surface(&self) -> Vec<Box<dyn crate::surface::Surface>> {
        let mut surfaces = self
            .children
            .iter()
            .flat_map(|widget| widget.surface())
            .collect::<Vec<_>>();

        let mut surface = RectSurface::new(&self.id);
        surface.color(self.color.clone());

        surfaces.push(Box::new(surface));

        surfaces
    }

	fn primitive(&self) -> Primitive {
		Primitive::Rect { 
			id: &self.id, 
			corner_radius: 0, // TODO add corner radius 
			color: self.color 
		}	
	}

    fn update(&mut self) {
        self.children.iter_mut().for_each(|child| child.update());
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
