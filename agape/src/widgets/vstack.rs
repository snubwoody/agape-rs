use crate::style::BoxStyle;
use crate::widgets::{LayoutDescription, LayoutType, RenderBox, RenderObject};
use crate::{impl_style, widgets::Widget};
use agape_core::{GlobalId, Position, Size};
use agape_layout::{AxisAlignment, VerticalLayout};
use agape_renderer::Renderer;

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

    fn build(&self, renderer: &mut Renderer) -> RenderBox {
        let children = self.children.iter().map(|w| w.build(renderer)).collect();

        // FIXME: use style and test
        let layout_desc = LayoutDescription {
            padding: self.layout.padding,
            spacing: self.layout.spacing,
            intrinsic_size: self.layout.intrinsic_size,
            cross_axis_alignment: self.layout.cross_axis_alignment,
            main_axis_alignment: self.layout.main_axis_alignment,
            layout_type: LayoutType::VerticalLayout,
        };

        let render_object = RenderObject::Rect {
            color: self.style.background_color.clone(),
            border: self.style.border.clone(),
        };

        RenderBox {
            id: self.id,
            position: Position::default(),
            size: Size::default(),
            render_object,
            children,
            layout_desc,
        }
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
