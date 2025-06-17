use crate::{impl_style, widgets::Widget};
use crystal::{BlockLayout, Layout};
use helium_core::{Color, GlobalId, Rgba};


/// A container [`Widget`] that wraps its child
pub struct Container<W> {
    id: GlobalId,
    color: Color<Rgba>,
    child: W,
    corner_radius: u32,
    padding: u32,
}

impl<W> Container<W>
where
    W: Widget,
{
    pub fn new(child: W) -> Self {
        Container {
            id: GlobalId::new(),
            color: Color::rgb(255, 255, 255),
            child,
            corner_radius: 0,
            padding: 0,
        }
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    impl_style!();
}

impl<W> Widget for Container<W>
where
    W: Widget,
{
    fn id(&self) -> GlobalId {
        self.id
    }


    fn layout(&self) -> Box<dyn Layout> {
        let child_layout = self.child.layout();
        let mut layout = BlockLayout::new(child_layout);
        layout.id = self.id;
        layout.padding = self.padding;
        Box::new(layout)
    }


    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }
}
