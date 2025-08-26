//! [`Widget`]'s describe what you want to present onto the screen. Agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod container;
mod hstack;
pub mod image;
mod rect;
mod svg;
mod text;
mod vstack;

use crate::State;
use crate::message::MessageQueue;
use agape_core::GlobalId;
use agape_layout::Layout;
use agape_renderer::Renderer;
pub use container::Container;
pub use hstack::*;
pub use image::Image;
pub use rect::*;
pub use svg::Svg;
pub use text::Text;
pub use vstack::*;

/// A [`View`].
///
/// # Example
/// ```no_run
/// use agape::{widgets::*,App};
///
/// #[derive(Default)]
/// struct TextBox{
///     text: String
/// }
///
/// impl View for TextBox{
///     type Widget = Text;
///     fn view(&self) -> Text{
///         Text::new(&self.text)
///     }
/// }
///
/// fn main() -> agape::Result<()>{
///     App::new(TextBox::default())
///         .run()
/// }
/// ```
///
/// The [`update`] method runs every frame and enables you to respond
/// to state changes and events.
///
/// [`update`]: View::update
pub trait View {
    type Widget: Widget;

    fn update(&mut self, _: &State, _: &mut MessageQueue) {}

    fn view(&self) -> Self::Widget;
}

pub trait Widget {
    /// Get the `id` of the [`Widget`]
    fn id(&self) -> GlobalId;

    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout>;

    /// Draw the widget to the screen.
    fn render(&self, _: &mut Renderer, _: &dyn Layout) {}
}
