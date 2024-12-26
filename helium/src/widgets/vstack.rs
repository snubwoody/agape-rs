use crate::{
    app::events::Event, impl_events, impl_style, 
	surface::rect::RectSurface, 
	widgets::{Widget, WidgetBody}, Color
};
use crystal::{BoxSizing, Layout, VerticalLayout};

pub struct VStack {
	pub id:String,
    pub children: Vec<Box<dyn Widget>>,
    pub color: Color,
	pub spacing:u32,
	pub padding:u32
}

impl VStack {
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

impl Widget for VStack {
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
			children:children_body,
			surface: Box::new(surface),
			..Default::default()
		};
	
		let mut layout = VerticalLayout::new();
		layout.intrinsic_size.height = BoxSizing::Flex(1);
		layout.children = children_layout;
		layout.id = body.id.clone();
		layout.padding = self.padding;
		layout.spacing = self.spacing;

		(body,Box::new(layout))
    }
}


// TODO test these macros pls
#[macro_export]
macro_rules! vstack {
	($($child:expr),*) => {
		{
			$crate::widgets::VStack{
				id:helium::nanoid!(),
				color:$crate::TRANSPARENT,
				spacing:0,
				padding:0,
				children:vec![
					$(
						Box::new($child),
					)*
				]
			}
		}
		
	};
}
