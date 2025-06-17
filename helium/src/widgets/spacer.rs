use super::Widget;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::GlobalId;

/// A [`Widget`] that fills up all the available space.  
///
/// The most common usage is to push [`Widget`]'s to the end of its
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
    id: GlobalId,
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            id: GlobalId::new(),
        }
    }
}

// TODO widgets might be easier to test now

impl Widget for Spacer {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn layout(&self) -> Box<dyn crystal::Layout> {
        let mut layout = EmptyLayout::new();
        layout.id = self.id;
        layout.intrinsic_size.width = BoxSizing::Flex(1);
        layout.intrinsic_size.height = BoxSizing::Flex(1);

        Box::new(layout)
    }
}
