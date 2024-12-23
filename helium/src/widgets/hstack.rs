use crystal::{BoxSizing, HorizontalLayout, IntrinsicSize};
use crate::{
    app::events::Event, impl_events, impl_style, 
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

		let intrinsic_size = IntrinsicSize {
			width: BoxSizing::Flex(1),
			height: BoxSizing::Shrink,
		};

		let mut layout = self.layout;
		layout.intrinsic_size(intrinsic_size);

        let children = self
            .children
            .iter()
            .map(|widget| Box::new(widget.build()))
            .collect();


        WidgetBody {
			id:self.id.clone(),
            children,
            layout: Box::new(layout),
            surface: Box::new(surface),
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
				]
			}
		}
		
	};
}
