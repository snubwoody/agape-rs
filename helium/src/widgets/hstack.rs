use crate::{
    app::events::Event,
    impl_events, impl_style, impl_widget,
    surface::rect::RectSurface,
    widgets::{Widget, WidgetBody},
    Color,
};
use crystal::{AxisAlignment, HorizontalLayout, Layout};
use helium_core::color::TRANSPARENT;

pub struct HStack {
    pub children: Vec<Box<dyn Widget>>,
    pub color: Color,
    pub layout: HorizontalLayout,
}

impl HStack {
    pub fn new() -> Self {
        HStack {
            color: TRANSPARENT,
            children: vec![],
            layout: HorizontalLayout::new(),
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

// TODO test this
impl Widget for HStack {
    fn build(&self) -> (WidgetBody, Box<dyn Layout>) {
		let id = nanoid::nanoid!();
        let mut surface = RectSurface::new(&id);
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
            id: id.clone(),
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
            id: id.clone(),
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

    fn update(&mut self) {
        self.children.iter_mut().for_each(|child| child.update());
    }
}

// TODO add array style syntax like the vec! macro
/// Creates an [`HStack`].  
/// `hstack!` allows [`HStack`]'s to be declared in a more declarative manner.
/// ```ignore
///
/// hstack!{
/// 	Button::new("Click me"),
/// 	Text::new("Hello world")
/// }
/// ```
#[macro_export]
macro_rules! hstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::HStack{
				color:$crate::TRANSPARENT,
				layout:$crate::HorizontalLayout::new(),
				children:vec![
					$(
						Box::new($child),
					)*
				]
			}
		}

	};
}
