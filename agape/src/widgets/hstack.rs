use crate::style::BoxStyle;
use crate::view::{RectView, View};
use crate::{impl_style, widgets::Widget};
use agape_core::GlobalId;
use agape_layout::{AxisAlignment, HorizontalLayout, Layout};

/// A horizontal stack of widgets, placed one after another.
///
/// `Hstack`s will most commonly be used with the [`hstack!`] macro
/// as a more convenient way to construct them.
///
/// ```
/// use agape::{hstack,widgets::{Rect,Text}};
///
/// let hstack = hstack! {
///     Rect::new().fill(),
///     Text::new("Hi there!"),
/// };
/// ```
///
/// You can, as well, construct them manually.
///
/// ```
/// use agape::widgets::{HStack,Text};
///
/// let mut hstack = HStack::new()
///     .add_child(Text::new("Hello "))
///     .add_child(Text::new("world!"));
/// ```
#[derive(Default)]
pub struct HStack {
    id: GlobalId,
    children: Vec<Box<dyn Widget>>,
    layout: HorizontalLayout,
    style: BoxStyle,
}

impl HStack {
    pub fn new() -> Self {
        HStack {
            id: GlobalId::default(),
            children: vec![],
            layout: HorizontalLayout::new(),
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

// TODO test this
impl Widget for HStack {
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

        let HorizontalLayout {
            spacing,
            padding,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            ..
        } = self.layout;

        let layout = HorizontalLayout {
            id: self.id,
            spacing,
            padding,
            intrinsic_size: self.style.intrinsic_size,
            cross_axis_alignment,
            main_axis_alignment,
            constraints,
            children: children_layout,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children.iter().map(|child| child.as_ref()).collect()
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        self.children.as_mut_slice()
    }
}

/// Creates an [`HStack`].  
///
/// `hstack!` allows [`HStack`]'s to be declared in a more declarative manner.
///
/// - Create an [`Hstack`] from a list of widgets.
/// ```
/// use agape::{hstack,widgets::{Rect}};
///
/// hstack!{
///     Rect::new(),
///     Rect::new(),
/// }
/// .spacing(12)
/// .padding(24);
///
/// ```
///
/// - Create an [`Hstack`] from a given widget and size.
/// ```
/// use agape::hstack;
/// use agape::widgets::Rect;
///
/// let hstack = hstack![Rect::new();10];
/// ```
///
/// > Note that to use the repeat syntax the [`Widget`] must implement
/// > `Clone`.
#[macro_export]
macro_rules! hstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::HStack::new()
			$(.add_child($child))*
		}
	};
    ($child:expr;$count:expr) => {
        {
            let mut hstack = $crate::widgets::HStack::new();
            for _ in 0..$count {
                hstack = hstack.add_child($child.clone());
            }
            hstack
        }
    };
	()=>{
		$crate::widgets::HStack::new()
	}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::{Rect, Text};
    use agape_layout::BoxSizing;

    #[test]
    fn layout_properties() {
        let hstack = HStack::new().fill_width();

        let layout = hstack.layout();

        assert_eq!(layout.intrinsic_size().width, BoxSizing::Flex(1));
    }

    #[test]
    fn traverse_children() {
        let mut hstack = hstack! {
            Text::new("Hello"),
            Text::new("Hello"),
            Text::new("Hello"),
        };

        let mut length = 0;
        hstack.traverse(&mut |_| {
            length += 1;
        });
        assert_eq!(length, 3);

        hstack.traverse_mut(&mut |_| {
            length += 1;
        });

        assert_eq!(length, 6);
    }

    #[test]
    fn hstack_expansion() {
        let hstack = hstack! {
            Rect::new(),
            Rect::new(),
        };

        assert_eq!(hstack.children.len(), 2);
    }

    #[test]
    fn hstack_repeat_syntax() {
        let hstack = hstack! {Text::new("hello world");10};
        assert_eq!(hstack.children.len(), 10);
    }

    #[test]
    fn empty_hstack_expansion() {
        let hstack = hstack! {};
        assert!(hstack.children.is_empty());
    }

    #[test]
    fn get_children() {
        let widget = hstack! {
            Rect::new(),
            Rect::new()
        };

        let id1 = widget.children()[0].id();
        let id2 = widget.children()[1].id();

        let children = widget.children();

        assert_eq!(children[0].id(), id1);
        assert_eq!(children[1].id(), id2);
    }

    #[test]
    fn get_view() {
        let hstack = hstack! {};
        let view = hstack.view();
        assert_eq!(view.color(), &hstack.style.background_color);
        assert_eq!(view.id(), hstack.id);
    }
}
