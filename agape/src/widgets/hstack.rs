use crate::style::BoxStyle;
use crate::{impl_style, widgets::Widget};
use agape_core::GlobalId;
use agape_layout::{AxisAlignment, HorizontalLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;

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
            // TODO: replace with BoxStyle
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

    fn children(&self) -> Vec<&dyn Widget> {
        self.children.iter().map(|w| w.as_ref()).collect()
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        self.children.iter_mut().for_each(|w| {
            f(w.as_mut());
            w.traverse(f);
        })
    }

    fn layout(&self, renderer: &mut Renderer) -> Box<dyn Layout> {
        let children: Vec<Box<dyn Layout>> =
            self.children.iter().map(|w| w.layout(renderer)).collect();
        let layout = HorizontalLayout {
            id: self.id,
            intrinsic_size: self.style.intrinsic_size,
            main_axis_alignment: self.layout.main_axis_alignment,
            cross_axis_alignment: self.layout.cross_axis_alignment,
            spacing: self.layout.spacing,
            padding: self.style.padding,
            children,
            ..Default::default()
        };

        Box::new(layout)
    }

    fn render(&self, renderer: &mut Renderer, layout: &dyn Layout) {
        let layout = layout.get(self.id).unwrap();
        let size = layout.size();
        let position = layout.position();
        // TODO: test this
        let mut rect = Rect::new()
            .size(size.width, size.height)
            .color(self.style.background_color.clone())
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
/// .padding_all(24.0);
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
    use crate::widgets::{Rect, Text, Widget};

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
    fn traverse() {
        let mut hstack = hstack![Text::default();3];
        let mut ids = vec![];
        hstack.traverse(&mut |w| {
            ids.push(w.id());
        });
        assert_eq!(hstack.children.len(), ids.len());
    }

    #[test]
    fn traverse_nested_children() {
        let mut hstack = hstack![hstack![Text::default()], hstack![Rect::new(),]];
        let mut ids = vec![];
        hstack.traverse(&mut |w| {
            ids.push(w.id());
        });
        assert_eq!(ids.len(), 4);
    }
}
