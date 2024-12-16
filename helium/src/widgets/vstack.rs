use crate::{
    app::events::Event, impl_events, impl_style, 
	layout::{IntrinsicSize,VerticalLayout, WidgetSize}, 
	surface::rect::RectSurface, 
	widgets::{Widget, WidgetBody}, Color
};

pub struct VStack {
	pub id:String,
    pub children: Vec<Box<dyn Widget>>,
    pub layout: VerticalLayout,
    pub color: Color,
	pub intrinsic_size:IntrinsicSize
}

impl VStack {
	pub fn fill_height(mut self) -> Self{
		self.intrinsic_size.fill_height();
		self
	}

	pub fn spacing(mut self, spacing: u32) -> Self {
		self.layout.spacing(spacing);
		self
	}

	pub fn padding(mut self,padding:u32) -> Self{
		self.layout.padding(padding);
		self
	}

	impl_style!();
	impl_events!();
}

impl Widget for VStack {
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
            layout: Box::new(self.layout),
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
			$crate::widgets::VStack{
				id:helium::nanoid!(),
				color:$crate::TRANSPARENT,
				layout:$crate::layout::VerticalLayout::new(0,0),
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
