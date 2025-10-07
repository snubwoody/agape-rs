use crate::element::{Element, ElementKind, LayoutKind};
use crate::state::Scroll;
use crate::style::BoxStyle;
use crate::{MessageQueue, impl_style, widgets::Widget};
use agape_core::GlobalId;
use agape_layout::{AxisAlignment, Layout, VerticalLayout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;

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

// TODO: add children method that takes into iterator
impl VStack {
    pub fn new() -> Self {
        VStack {
            id: GlobalId::new(),
            children: vec![],
            layout: VerticalLayout::new(),
            style: BoxStyle::default(),
        }
    }

    /// Removes the last child of the vstack and returns it.
    pub fn pop(&mut self) -> Option<Box<dyn Widget>> {
        self.children.pop()
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Clear the vstack, removing all children.
    pub fn clear(&mut self) {
        self.children.clear();
    }

    pub fn get(&self, index: usize) -> Option<&dyn Widget> {
        self.children.get(index).map(|w| &**w)
    }

    pub fn append_child(&mut self, child: impl Widget + 'static) {
        self.children.push(Box::new(child));
    }

    pub fn add_child(mut self, widget: impl Widget + 'static) -> Self {
        self.children.push(Box::new(widget));
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.style.padding = padding;
        self
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.style.spacing = spacing;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.style.main_axis_alignment = AxisAlignment::Center;
        self.style.cross_axis_alignment = AxisAlignment::Center;
        self
    }

    pub fn main_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.style.main_axis_alignment = alignment;
        self
    }

    pub fn cross_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.style.cross_axis_alignment = alignment;
        self
    }

    impl_style!();
}

impl Widget for VStack {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn build(&self) -> Element {
        let children = self.children.iter().map(|w| w.build()).collect();
        Element {
            id: self.id,
            kind: ElementKind::Rect {
                style: self.style.clone(),
                layout: LayoutKind::Vertical,
            },
            on_click: None,
            children,
        }
    }

    fn tick(&mut self, messages: &mut MessageQueue) {
        if let Some(scroll) = messages.get::<Scroll>() {
            self.layout.scroll(scroll.0);
        }
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        self.children.iter_mut().for_each(|w| {
            f(w.as_mut());
            w.traverse(f);
        })
    }

    fn children(&self) -> Vec<&dyn Widget> {
        self.children.iter().map(|w| w.as_ref()).collect()
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
    use crate::widgets::{Rect, Text, Widget};

    #[test]
    fn vstack_expansion() {
        let vstack = vstack! {
            Rect::new(),
            Rect::new(),
            Rect::new(),
        };

        assert_eq!(vstack.children.len(), 3);
    }

    #[test]
    fn vstack_repeat_syntax() {
        let vstack = vstack![Text::new("Repeat!");10];
        assert_eq!(vstack.children.len(), 10);
    }

    #[test]
    fn traverse() {
        let mut vstack = vstack![Text::default();3];
        let mut ids = vec![];
        vstack.traverse(&mut |w| {
            ids.push(w.id());
        });
        assert_eq!(vstack.children.len(), ids.len());
    }

    #[test]
    fn traverse_nested_children() {
        let mut vstack = vstack![vstack![Text::default()], vstack![Rect::new(),]];
        let mut ids = vec![];
        vstack.traverse(&mut |w| {
            ids.push(w.id());
        });
        assert_eq!(ids.len(), 4);
    }
}
