use tokio::time::Sleep;

use crate::{
    app::events::{self, Event, Signal}, Color, impl_events, layout::{IntrinsicSize, Layout, WidgetSize}, surface::rect::RectSurface, widgets::{Widget, WidgetBody}
};

pub struct Stack {
	pub id:String,
    pub spacing: u32,
    pub padding: u32,
    pub children: Vec<Box<dyn Widget>>,
    pub layout: Layout,
    pub color: Color,
    pub events: Vec<Event>,
}

impl Stack {
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        
		match self.layout {
			Layout::Horizontal { padding,.. } => {
				self.layout = Layout::Horizontal {
					spacing:self.spacing,
					padding,
				};
			},
			Layout::Vertical { padding,.. } => {
				self.layout = Layout::Vertical {
					spacing:self.spacing,
					padding,
				};
			}, 
			_ => {}
		}

        self
    }

	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
        
		match self.layout {
			Layout::Horizontal { spacing,.. } => {
				self.layout = Layout::Horizontal {
					spacing,
					padding: self.padding,
				};
			},
			Layout::Vertical { spacing, .. } => {
				self.layout = Layout::Vertical {
					spacing,
					padding: self.padding,
				};
			}, 
			_ => {}
		}

        self
	}

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
		helium::widgets::Stack{
			id:helium::nanoid!(),
			spacing:0,
			padding:0,
			color:helium::Color::Rgb(255,255,255),
			layout:helium::layout::Layout::Vertical {
				spacing:0,
				padding:0,
			},
			children:vec![
				$(
					Box::new($child),
				)*
			],
			events: Vec::new(),
		}
	};
}

#[macro_export]
macro_rules! hstack {
	($($child:expr),*) => {
		helium::widgets::Stack{
			id:helium::nanoid!(),
			spacing:0,
			padding:0,
			color: helium::Color::Rgb(255,255,255),
			layout: helium::layout::Layout::Horizontal {
				spacing:0,
				padding:0,
			},
			children:vec![
				$(
					Box::new($child),
				)*
			],
			events: Vec::new(),
		}
	};
}
