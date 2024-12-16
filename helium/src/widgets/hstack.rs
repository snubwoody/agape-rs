use crate::{
    app::events::Event, impl_events, impl_style, 
	layout::{HorizontalLayout, IntrinsicSize, WidgetSize}, 
	surface::rect::RectSurface, 
	widgets::{Widget, WidgetBody}, Color
};

// TODO make fields private
pub struct HStack {
	pub id:String,
    pub children: Vec<Box<dyn Widget>>,
    pub layout: HorizontalLayout,
    pub color: Color,
	pub intrinsic_size:IntrinsicSize
}

impl HStack {
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

impl Widget for HStack {
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
            intrinsic_size:self.intrinsic_size,
            ..Default::default()
        }
    }
}


// TODO test these macros pls
// TODO change the color path because it might conflict with local colors
#[macro_export]
macro_rules! hstack {
	($($child:expr),*) => {
		{
			$crate::widgets::HStack{
				id:helium::nanoid!(),
				color:$crate::TRANSPARENT,
				layout:$crate::layout::HorizontalLayout::new(0,0),
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
