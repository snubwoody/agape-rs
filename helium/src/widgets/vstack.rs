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
}

impl VStack {
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

		(body,Box::new(layout))
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
				children:vec![
					$(
						Box::new($child),
					)*
				]
			}
		}
		
	};
}
