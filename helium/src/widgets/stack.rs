use crate::{
    app::events::{self, Event, }, impl_events, impl_style, layout::{IntrinsicSize, Layout, WidgetSize}, surface::rect::RectSurface, widgets::{Widget, WidgetBody}, Color
};

// TODO make fields private
pub struct Stack {
	pub id:String,
    pub children: Vec<Box<dyn Widget>>,
    pub layout: Layout,
    pub color: Color,
}

impl Stack {
	impl_style!();
	impl_events!();
}

impl Widget for Stack {
    fn build(&self) -> WidgetBody {
        let mut surface = RectSurface::default();
        surface.color(self.color.clone());

        let children = self
            .children
            .iter()
            .map(|widget| Box::new(widget.build()))
            .collect();

        WidgetBody {
			id:self.id.clone(),
            children,
            layout: self.layout,
            surface: Box::new(surface),
            intrinsic_size: IntrinsicSize {
                width: WidgetSize::Fill,
                height: WidgetSize::Fit,
            },
            ..Default::default()
        }
    }
}

// TODO test these macros pls
#[macro_export]
macro_rules! vstack {
	($($child:expr),*) => {
		{
			let mut layout = helium::layout::Layout::vertical();
			helium::widgets::Stack{
				id:helium::nanoid!(),
				color:helium::Color::Rgb(255,255,255),
				layout,
				children:vec![
					$(
						Box::new($child),
					)*
				],
			}
		}
		
	};
}

#[macro_export]
macro_rules! hstack {
	($($child:expr),*) => {
		{
			let mut layout = $crate::layout::Layout::horizontal();
			$crate::widgets::Stack{
				id:$crate::nanoid!(),
				color: $crate::Color::Rgb(255,255,255),
				layout,
				children:vec![
					$(
						Box::new($child),
					)*
				],
			}
		}
		
	};
}