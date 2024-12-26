use crystal::{BoxSizing, HorizontalLayout, IntrinsicSize, Layout};
use crate::{
    app::events::Event, impl_events, impl_style, 
	surface::rect::RectSurface, 
	widgets::{Widget, WidgetBody}, Color
};

pub struct HStack {
	pub id:String,
    pub children: Vec<Box<dyn Widget>>,
    pub color: Color,
	pub intrinsic_size:IntrinsicSize,
	pub spacing:u32,
	pub padding:u32
}

impl HStack {
	pub fn spacing(mut self, spacing: u32) -> Self {
		self.spacing = spacing;
		self
	}
	
	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
		self
	}

	impl_style!();
	impl_events!();
}

// TODO test this
impl Widget for HStack {
    fn build(&self) -> (WidgetBody,Box<dyn Layout>) {
        let mut surface = RectSurface::default();
        surface.color(self.color.clone());
		
        let (children_body,children_layout):(Vec<Box<WidgetBody>>,Vec<Box<dyn Layout>>) = 
		self
		.children
		.iter()
		.map(|widget| {
			let (body,layout) = widget.build();
			return (Box::new(body),layout);
		})
		.collect();
		

		let body = WidgetBody {
			id:self.id.clone(),
            surface: Box::new(surface),
			children:children_body,
            ..Default::default()
        };
	
		let mut layout = HorizontalLayout::new();
		layout.intrinsic_size.width = BoxSizing::Flex(1);
		layout.children = children_layout;
		layout.id = body.id.clone();
		layout.spacing = self.spacing;
		layout.padding = self.padding;

		(body,Box::new(layout))
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
				padding:0,
				spacing:0,
				intrinsic_size:$crate::IntrinsicSize::default(),
				children:vec![
					$(
						Box::new($child),
					)*
				]
			}
		}
		
	};
}
