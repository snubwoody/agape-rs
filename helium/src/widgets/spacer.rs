use super::Widget;
use crystal::{BoxSizing, EmptyLayout};
use helium_renderer::Rect;

/// A [`Widget`] that fills up all the available space.  
///
/// The most common usage is to push [`Widget`]'s to the end of it's
/// parent.
///
/// ```
/// use helium::{hstack,widgets::{Spacer,Text,Button}};
///
/// let navbar = hstack!{
/// 	Text::new("Logo"),
/// 	Text::new("Shop"),
/// 	Text::new("Contact us"),
/// 	Spacer::new(),
/// 	Button::text("Cart"),
/// };
/// ```
/// **Note** that `Spacer`'s have no effect when the parent `widget` has
/// an intrinsic size of `Shrink`, because the parent will try to be
/// as small as possible, hence the spacer will have a size of 0.
pub struct Spacer {
    id: String,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(),
        }
    }
}

impl Widget for Spacer {
    fn id(&self) -> &str {
        &self.id
    }

    fn layout(&self, _: &mut helium_renderer::Renderer) -> Box<dyn crystal::Layout> {
        let mut layout = EmptyLayout::new();
        layout.id = self.id.clone();
        layout.intrinsic_size.width = BoxSizing::Flex(1);
        layout.intrinsic_size.height = BoxSizing::Flex(1);

        Box::new(layout)
    }

    fn draw(&self, layout: &dyn crystal::Layout, renderer: &mut helium_renderer::Renderer) {
        renderer.draw([Rect::new(layout.size().width, layout.size().height)
            .position(layout.position().x, layout.position().y)]);
    }
}
