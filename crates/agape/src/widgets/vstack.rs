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

    /// Append a [`Widget`] into the vstack.
    pub fn push(&mut self, child: impl Widget + 'static) {
        self.children.push(Box::new(child));
    }

    pub fn with_child(mut self, widget: impl Widget + 'static) -> Self {
        self.children.push(Box::new(widget));
        self
    }

    pub fn with_children<I, W>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = W>,
        W: Widget + 'static,
    {
        for child in iter {
            self.children.push(Box::new(child));
        }
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

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let children: Vec<Box<dyn Layout>> =
            self.children.iter().map(|w| w.layout(renderer)).collect();
        // TODO: join style and layout
        let layout = VerticalLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            main_axis_alignment: self.layout.main_axis_alignment,
            cross_axis_alignment: self.layout.cross_axis_alignment,
            spacing: self.layout.spacing,
            padding: self.style.padding,
            scroll_offset: self.layout.scroll_offset,
            children,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();
        let mut rect = Rect::new()
            .size(size.width, size.height)
            .position(position.x, position.y)
            .corner_radius(self.style.corner_radius);

        rect.border = self.style.border.clone();

        renderer.draw_rect(rect);
        // TODO: test this
        self.children
            .iter()
            .for_each(|child| child.render(renderer, layout));
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
			$(.with_child($child))*
		}

	};
    ($child:expr;$count:expr) => {
        {
            let mut vstack = $crate::widgets::VStack::new();
            for _ in 0..$count {
                vstack.push($child.clone());
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
