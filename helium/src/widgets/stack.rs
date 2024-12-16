use crate::{
    app::events::{self, Event, }, impl_events, impl_style, layout::{IntrinsicSize, Layout, WidgetSize}, surface::rect::RectSurface, widgets::{Widget, WidgetBody}, Color
};

// TODO make fields private
pub struct Stack {
	pub id:String,
    pub children: Vec<Box<dyn Widget>>,
    pub layout: Layout,
    pub color: Color,
	pub intrinsic_size:IntrinsicSize
}

impl Stack {
	pub fn fill_height(mut self) -> Self{
		self.intrinsic_size.fill_height();
		self
	}

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
// TODO change the color path because it might conflict with local colors
#[macro_export]
macro_rules! vstack {
	($($child:expr),*) => {
		{
			let mut layout = $crate::layout::Layout::vertical();
			$crate::widgets::Stack{
				id:helium::nanoid!(),
				color:$crate::TRANSPARENT,
				layout,
				children:vec![
					$(
						Box::new($child),
					)*
				],
				intrinsic_size:$crate::layout::IntrinsicSize {
					width: $crate::layout::WidgetSize::Fit,
					height: $crate::layout::WidgetSize::Fill,
				}
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
				color: $crate::TRANSPARENT,
				layout,
				children:vec![
					$(
						Box::new($child),
					)*
				],
				intrinsic_size:$crate::layout::IntrinsicSize {
					width: $crate::layout::WidgetSize::Fill,
					height: $crate::layout::WidgetSize::Fit,
				}
			}
		}
		
	};
}