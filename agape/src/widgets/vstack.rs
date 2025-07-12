use crate::style::BoxStyle;
use crate::view::{RectView, View};
use crate::{impl_style, widgets::Widget};
use agape_core::GlobalId;
use agape_layout::{AxisAlignment, Layout, VerticalLayout};

/// A vertical stack that places its children vertically one after
/// another.
///
/// ```
/// use agape::{vstack,widgets::Text};
///
/// let vstack = vstack!{
///     Text::new("Hello"),
///     Text::new("world!"),
/// };
/// ```
pub struct VStack {
    id: GlobalId,
    children: Vec<Box<dyn Widget>>,
    layout: VerticalLayout,
    pub style: BoxStyle,
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
            children: vec![],
            layout: VerticalLayout::new(),
            style: BoxStyle::default(),
        }
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

    impl_style!();
}

impl Widget for VStack {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn traverse(&self, f: &mut dyn FnMut(&dyn Widget)) {
        for child in &self.children {
            f(child.as_ref());
            child.traverse(f);
        }
    }

    fn traverse_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        for child in &mut self.children {
            f(child.as_mut());
            child.traverse_mut(f);
        }
    }

    fn view(&self) -> Box<dyn View> {
        let view = RectView {
            id: self.id,
            color: self.style.background_color.clone(),
            border: self.style.border.clone(),
            ..Default::default()
        };
        Box::new(view)
    }

    fn layout(&self) -> Box<dyn Layout> {
        let children_layout: Vec<Box<dyn Layout>> =
            self.children.iter().map(|widget| widget.layout()).collect();

        let VerticalLayout {
            spacing,
            padding,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            scroll_offset,
            ..
        } = self.layout;

        let layout = VerticalLayout {
            id: self.id,
            spacing,
            padding,
            intrinsic_size: self.style.intrinsic_size,
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

/// Creates a [`Vstack`].
///
/// - Create a [`Vstack`] from a given list of widgets.
/// ```
/// use agape::vstack;
/// use agape::widgets::Text;
///
/// let vstack = vstack!{
///     Text::new("Hello"),
///     Text::new("world!"),
/// }
/// .spacing(12)
/// .align_center()
/// .fill();
/// ```
///
/// - Create a [`Vstack`] from a widget and a size.
/// ```
/// use agape::vstack;
/// use agape::widgets::*;
///
/// let vstack = vstack![Text::new("Repeat!");100];
/// assert_eq!(vstack.children().len(),100);
/// ```
///
/// > Note that to use the repeat syntax the [`Widget`] must
/// > implement `Clone`.
#[macro_export]
macro_rules! vstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::VStack::new()
			$(.add_child($child))*
		}

	};
    ($child:expr;$count:expr) => {
        {
            let mut vstack = $crate::widgets::VStack::new();
            for _ in 0..$count {
                vstack = vstack.add_child($child.clone());
            }
            vstack
        }
    };
	()=>{
		$crate::widgets::VStack::new()
	};
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::{Rect, Text};
    use agape_layout::BoxSizing;

    #[test]
    fn layout_properties() {
        let vstack = VStack::new().fixed(100.0, 200.0);

        let layout = vstack.layout();
        assert_eq!(layout.intrinsic_size().width, BoxSizing::Fixed(100.0));
        assert_eq!(layout.intrinsic_size().height, BoxSizing::Fixed(200.0));
    }

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
    fn vstack_repeat_syntax() {
        let vstack = vstack![Text::new("Repeat!");10];
        assert_eq!(vstack.children.len(), 10);
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

        assert_eq!(view.color(), &vstack.style.background_color);
        assert_eq!(view.id(), vstack.id());
    }
}
