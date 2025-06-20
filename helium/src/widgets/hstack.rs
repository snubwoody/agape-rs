use crate::{Color, impl_layout, impl_style, widgets::Widget};
use crystal::{AxisAlignment, HorizontalLayout, Layout};
use helium_core::{GlobalId, Rgba, colors::TRANSPARENT};

pub struct HStack {
    id: GlobalId,
    children: Vec<Box<dyn Widget>>,
    color: Color<Rgba>,
    corner_radius: u32,
    layout: HorizontalLayout,
}

// TODO add get methods
impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}

impl HStack {
    pub fn new() -> Self {
        HStack {
            id: GlobalId::default(),
            color: TRANSPARENT,
            children: vec![],
            corner_radius: 0,
            layout: HorizontalLayout::new(),
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

// TODO test this
impl Widget for HStack {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn layout(&self) -> Box<dyn Layout> {
        let children_layout: Vec<Box<dyn Layout>> =
            self.children.iter().map(|widget| widget.layout()).collect();

        let HorizontalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            ..
        } = self.layout;

        // TODO use builder pattern?
        let layout = HorizontalLayout {
            id: self.id,
            spacing,
            padding,
            intrinsic_size,
            cross_axis_alignment,
            main_axis_alignment,
            constraints,
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

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        self.children.as_mut_slice()
    }
}

/// Creates an [`HStack`].  
///
/// `hstack!` allows [`HStack`]'s to be declared in a more declarative manner.
///
/// ```
/// use helium::{hstack,widgets::{Rect}};
///
/// hstack!{
///     Rect::new(100.0,200.0),
///     Rect::new(100.0,200.0),
/// };
///
/// ```
#[macro_export]
macro_rules! hstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::HStack::new()
			$(.add_child($child))*
		}
	};


	()=>{
		$crate::widgets::HStack::new()
	};
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::{Circle, Rect};

    #[test]
    fn hstack_expansion() {
        let hstack = hstack! {
            Rect::new(200.0,100.0),
            Rect::new(200.0,100.0),
        };

        assert_eq!(hstack.children.len(), 2);
    }

    #[test]
    fn empty_hstack_expansion() {
        let hstack = hstack! {};
        assert!(hstack.children.is_empty());
    }

    #[test]
    fn get_children() {
        let widget = hstack! {
            Rect::new(100.0,100.0),
            Circle::new(100)
        };
        let id1 = widget.children()[0].id();
        let id2 = widget.children()[1].id();

        let children = widget.children();

        assert_eq!(children[0].id(), id1);
        assert_eq!(children[1].id(), id2);
    }
}
