//! A [`Widget`] describe what you want to present onto the screen. `agape` tries to
//! provide as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],
//! [`HStack`] and [`VStack`], and so on. There are two ways of creating widgets.
mod button;
mod container;
mod hstack;
mod icon;
pub mod image;
mod rect;
mod svg;
mod text;
mod text_field;
mod vstack;

use crate::assets::AssetManager;
use crate::message::MessageQueue;
use crate::state::{Context, State, StateMap};
use agape_core::GlobalId;
use agape_layout::Layout;
use agape_renderer::Renderer;
pub use button::*;
pub use container::Container;
pub use hstack::*;
pub use icon::Icon;
pub use image::Image;
pub use rect::*;
pub use svg::Svg;
pub use text::Text;
pub use text_field::TextField;
pub use vstack::*;
// FIXME: start removing the weeds

/// A `Widget` is anything that can ultimately be drawn to the screen. Widgets internally
/// can be composed of anything, but each widget must have a [`GlobalId`] and a [`Layout`].
/// See the [module docs] for more info.
///
/// [module docs]: crate::widgets
pub trait Widget {
    /// Get the `id` of the [`Widget`].
    fn id(&self) -> GlobalId;

    fn state(&self, index: usize, state_map: &mut StateMap) {}
    fn get_state(&mut self, index: usize, state_map: &mut StateMap) {}

    /// Construct a [`Layout`] to solve layout for the whole
    /// widget tree.
    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout>;

    /// Draw the widget to the screen.
    fn render(&self, _: &mut Renderer, _: &dyn Layout);

    fn children(&self) -> Vec<&dyn Widget>;

    /// Traverse the widgets children. Note that this doesn't
    /// include the widget itself.
    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget));

    fn get_assets(&mut self, _: &AssetManager) {}

    /// Called every frame.
    fn tick(&mut self, _: &mut MessageQueue) {}
    fn click(&mut self, _: &mut MessageQueue) {}
    fn hover(&mut self, _: &mut MessageQueue) {}

    fn mouse_entered(&mut self, _: &mut MessageQueue) {}
    fn mouse_left(&mut self, _: &mut MessageQueue) {}
}

pub trait View {
    type Widget: Widget;

    fn update(&mut self, _: &mut MessageQueue) {}

    fn view(&self, ctx: &mut Context) -> Self::Widget;
}

/// An iterator over a tree of widgets.
pub struct WidgetIter<'a> {
    stack: Vec<&'a dyn Widget>,
}

impl<'a> Iterator for WidgetIter<'a> {
    type Item = &'a dyn Widget;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(widget) = self.stack.pop() {
            self.stack.extend(widget.children());
            return Some(widget);
        }
        None
    }
}

impl dyn Widget {
    pub fn iter(&self) -> WidgetIter<'_> {
        WidgetIter { stack: vec![self] }
    }
}
