use crate::{
     app::events::Event, color::Color, impl_events, impl_interative, layout::{IntrinsicSize, Layout, WidgetSize}, surface::rect::RectSurface, widgets::{Widget, WidgetBody}
};

pub struct Stack {
    pub spacing: u32,
    pub padding: u32,
    pub children: Vec<Box<dyn Widget>>,
	pub layout: Layout,
    pub color: Color,
	pub events: Vec<Event>
}

impl Stack {
	// FIXME change this to use the layout of the widget
	pub fn spacing(mut self,spacing:u32) -> Self{
		self.spacing = spacing;
		self.layout = Layout::Horizontal {
			spacing,
			padding:self.padding,
		};
		self
	}
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
			children,
            layout:self.layout,
            surface: Box::new(surface),
			intrinsic_size:IntrinsicSize { width: WidgetSize::Fill, height: WidgetSize::Fit },
            ..Default::default()
        }
    }

    fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
        self.children
    }

	fn get_children_ref(&self) -> Vec<&Box<dyn Widget>> {
		self.children.iter().map(|child|child).collect()
	}
}




#[macro_export]
macro_rules! vstack {
	(
		spacing:$spacing:expr,
		padding:$padding:expr,
		$($child:expr),* // TODO remove this, i think it's not neccessary
	) => {
		helium::widgets::Stack{
			spacing:$spacing,
			padding:$padding,
			color:helium::color::Color::Rgb(255,255,255),
			layout:helium::layout::Layout::Vertical {
				spacing: $spacing,
				padding: $padding,
			},
			children:vec![
				$(
					Box::new($child),
				)*
			]
		}
	};
	($($child:expr),*) => {
		helium::widgets::Stack{
			spacing:0,
			padding:0,
			color:helium::color::Color::Rgb(255,255,255),
			layout:helium::layout::Layout::Vertical {
				spacing:0,
				padding:0,
			},
			children:vec![
				$(
					Box::new($child),
				)*
			]
		}
	};
}

#[macro_export]
macro_rules! hstack {
	(
		spacing:$spacing:expr,
		padding:$padding:expr,
		$($child:expr),*
	) => {
		helium::widgets::Stack{
			spacing:$spacing,
			padding:$padding,
			color:helium::color::Color::Rgb(255,255,255),
			layout:helium::layout::Layout::Horizontal {
				spacing: $spacing,
				padding: $padding,
			},
			children:vec![
				$(
					Box::new($child),
				)*
			]
		}
	};
	($($child:expr),*) => {
		helium::widgets::Stack{
			spacing:0,
			padding:0,
			color: helium::color::Color::Rgb(255,255,255),
			layout: helium::layout::Layout::Horizontal {
				spacing:0,
				padding:0,
			},
			children:vec![
				$(
					Box::new($child),
				)*
			]
		}
	};
}