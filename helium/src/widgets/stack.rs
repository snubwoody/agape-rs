use tokio::time::Sleep;

use crate::{
    app::events::{self, Event, Signal}, impl_events, impl_style, layout::{IntrinsicSize, Layout, WidgetSize}, surface::rect::RectSurface, widgets::{Widget, WidgetBody}, Color
};

pub struct Stack {
	pub id:String,
    pub children: Vec<Box<dyn Widget>>,
    pub layout: Layout,
    pub color: Color,
    pub events: Vec<Event>,
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

    fn get_children(self: Box<Self>) -> Vec<Box<dyn Widget>> {
        self.children
    }

    fn get_children_ref(&self) -> Vec<&Box<dyn Widget>> {
        self.children.iter().map(|child| child).collect()
    }

	fn process_signal(&mut self,signal:&Signal) {
		
		match signal {
			Signal::Click(id) =>{
				if id == &self.id{
					for event in self.events.iter_mut(){
						match event {
							Event::OnClick(func) => func(),
							_ => {}
						}
					}
				}
			}
			Signal::Hover(id) => {
				if id == &self.id{
					for event in self.events.iter_mut(){
						match event {
							Event::OnHover(func)=> func(),
							_ => {}
						}
					}
				}
			}
		}
	}
}

#[macro_export]
macro_rules! vstack {
	($($child:expr),*) => {
		{
			let mut layout =helium::layout::Layout::new();
			layout.layout(helium::layout::LayoutType::Vertical);

			helium::widgets::Stack{
				id:helium::nanoid!(),
				color:helium::Color::Rgb(255,255,255),
				layout,
				children:vec![
					$(
						Box::new($child),
					)*
				],
				events: Vec::new(),
			}
		}
		
	};
}

#[macro_export]
macro_rules! hstack {
	($($child:expr),*) => {
		{
			let mut layout = helium::layout::Layout::new();
			layout.layout(helium::layout::LayoutType::Horizontal);
			helium::widgets::Stack{
				id:helium::nanoid!(),
				color: helium::Color::Rgb(255,255,255),
				layout,
				children:vec![
					$(
						Box::new($child),
					)*
				],
				events: Vec::new(),
			}
		}
		
	};
}