use crate::{
    colour::Colour,
    layout::{IntrinsicSize, Layout, WidgetSize},
    surface::rect::RectSurface,
    widgets::{Widget, WidgetBody},
};

#[derive(Debug)]
pub struct VStack {
    pub spacing: u32,
    pub padding: u32,
    pub children: Vec<Box<dyn Widget>>,
	pub colour: Colour
}

impl Widget for VStack {
    fn build(&self) -> WidgetBody {
		let mut surface = RectSurface::default();
        surface.colour(self.colour.clone());
        let layout = Layout::Vertical {
            spacing: self.spacing,
            padding: self.padding,
        };

        let children = self
            .children
            .iter()
            .map(|widget| Box::new(widget.build()))
            .collect();

        WidgetBody {
            layout,
            children,
			surface: Box::new(surface),
			intrinsic_size: IntrinsicSize { width: WidgetSize::Fit, height: WidgetSize::Fill },
            ..Default::default()
        }
    }

    fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
        self.children
    }
}

#[derive(Debug)]

pub struct HStack {
    pub spacing: u32,
    pub padding: u32,
    pub children: Vec<Box<dyn Widget>>,
    pub colour: Colour,
}

impl Widget for HStack {
    fn build(&self) -> WidgetBody {
        let mut surface = RectSurface::default();
        surface.colour(self.colour.clone());
        let layout = Layout::Horizontal {
            spacing: self.spacing,
            padding: self.padding,
        };

        let children = self
            .children
            .iter()
            .map(|widget| Box::new(widget.build()))
            .collect();

        WidgetBody {
            layout,
            children,
            surface: Box::new(surface),
			intrinsic_size:IntrinsicSize { width: WidgetSize::Fill, height: WidgetSize::Fit },
            ..Default::default()
        }
    }

    fn get_children(self:Box<Self>) -> Vec<Box<dyn Widget>> {
        self.children
    }
}
