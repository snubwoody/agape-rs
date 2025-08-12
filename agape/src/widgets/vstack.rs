use crate::style::BoxStyle;
use crate::widgets::{LayoutDescription, LayoutType, RenderBox, RenderObject};
use crate::{impl_style, widgets::Widget};
use agape_core::{GlobalId, Position, Size};
use agape_layout::{AxisAlignment, HorizontalLayout, Layout, VerticalLayout};
use agape_renderer::Renderer;
use tiny_skia::Pixmap;

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

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let children: Vec<Box<dyn Layout>> =
            self.children.iter().map(|w| w.layout(renderer)).collect();
        let layout = VerticalLayout {
            id: self.id,
            intrinsic_size: self.layout.intrinsic_size,
            main_axis_alignment: self.layout.main_axis_alignment,
            cross_axis_alignment: self.layout.cross_axis_alignment,
            spacing: self.layout.spacing,
            children,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn render(&self, pixmap: &mut Pixmap, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();
        renderer.draw_rect(
            pixmap,
            &self.style.background_color.clone(),
            size,
            position,
            self.style.border.clone(),
        );
        // TODO: test this
        self.children
            .iter()
            .for_each(|child| child.render(pixmap, renderer, layout));
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
    use crate::widgets::{Rect, Text};

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
}
