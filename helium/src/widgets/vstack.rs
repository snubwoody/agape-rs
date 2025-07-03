use crate::view::{RectView, View};
use crate::{Color, impl_layout, impl_style, widgets::Widget};
use crystal::{AxisAlignment, Layout, VerticalLayout};
use helium_core::{GlobalId, Rgba};

pub struct VStack {
    id: GlobalId,
    children: Vec<Box<dyn Widget>>,
    color: Color<Rgba>,
    layout: VerticalLayout,
    corner_radius: u32,
}

impl Default for VStack {
    fn default() -> Self {
        Self::new()
    }
}

impl VStack {
    pub fn new() -> Self {
        VStack {
            id: GlobalId::new(),
            color: Color::TRANSPARENT,
            children: vec![],
            layout: VerticalLayout::new(),
            corner_radius: 0,
        }
    }

    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn get(&self, index: usize) -> Option<&dyn Widget> {
        self.children.get(index).map(|w| &**w)
    }

    pub fn add_child(mut self, widget: impl Widget + 'static) -> Self {
        self.children.push(Box::new(widget));
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.layout.padding = padding;
        self
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.layout.spacing = spacing;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.layout.main_axis_alignment = AxisAlignment::Center;
        self.layout.cross_axis_alignment = AxisAlignment::Center;
        self
    }

    pub fn main_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.main_axis_alignment = alignment;
        self
    }

    pub fn cross_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.cross_axis_alignment = alignment;
        self
    }

    impl_layout!();
    impl_style!();
}

impl Widget for VStack {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn view(&self) -> Box<dyn View> {
        let mut view = RectView::new(self.color.clone());
        view.set_id(self.id);
        Box::new(view)
    }

    fn layout(&self) -> Box<dyn Layout> {
        let children_layout: Vec<Box<dyn Layout>> =
            self.children.iter().map(|widget| widget.layout()).collect();

        let VerticalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            scroll_offset,
            ..
        } = self.layout;

        // TODO use builder pattern?
        let layout = VerticalLayout {
            id: self.id,
            spacing,
            padding,
            intrinsic_size,
            cross_axis_alignment,
            main_axis_alignment,
            constraints,
            scroll_offset,
            children: children_layout,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children
            .iter()
            .map(|child| child.as_ref())
            .collect::<Vec<_>>()
    }
}

#[macro_export]
macro_rules! vstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::VStack::new()
			$(.add_child($child))*
		}

	};

	()=>{
		$crate::widgets::VStack::new()
	};
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;

    #[test]
    fn vstack_expansion() {
        let vstack = vstack! {
            Rect::new(100.0,100.0),
            Rect::new(100.0,100.0),
            Rect::new(100.0,100.0),
        };

        assert_eq!(vstack.children.len(), 3);
    }

    #[test]
    fn get_children() {
        let vstack = vstack! {
            Rect::new(100.0,100.0),
            Rect::new(100.0,100.0),
        };

        let id1 = vstack.children()[0].id();
        let id2 = vstack.children()[1].id();

        let children = vstack.children();

        assert_eq!(id1, children[0].id());
        assert_eq!(id2, children[1].id());
    }

    #[test]
    fn get_view() {
        let vstack = vstack! {};
        let view = vstack.view();

        assert_eq!(view.color(), &vstack.color);
        assert_eq!(view.id(), vstack.id());
    }
}
