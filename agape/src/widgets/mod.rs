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

use crate::LayoutTree;
use crate::message::MessageQueue;
use crate::resources::CursorPosition;
use agape_core::GlobalId;
use agape_layout::Layout;
use agape_renderer::Renderer;
use bevy_ecs::prelude::*;
pub use container::Container;
pub use hstack::*;
pub use image::Image;
pub use rect::*;
use std::any::Any;
use std::collections::HashMap;
pub use svg::Svg;
pub use text::Text;
pub use vstack::*;

#[derive(Resource)]
pub(crate) struct ViewTree(pub Box<dyn View>);
#[derive(Resource)]
pub(crate) struct WidgetTree(pub Box<dyn Widget>);

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
///     fn view(&self) -> Box<dyn Widget>{
///         Box::new(Text::new(&self.text))
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
pub trait View: Send + Sync {
    // TODO: add const label
    fn update(&mut self, _: &mut MessageQueue) {}

    fn view(&self) -> Box<dyn Widget>;

    fn handle_hover(&mut self, layout: &dyn Layout) {}
}

// TODO: add label on view as an option then get from
// widget if None

/// A `Widget` is anything that can ultimately be drawn to the screen. Widgets internally
/// can be composed of anything, but each widget must have a [`GlobalId`] and a [`Layout`].
pub trait Widget: Send + Sync {
    /// Get the `id` of the [`Widget`]
    fn id(&self) -> GlobalId;

    /// Construct a [`Layout`] to solve layout for the whole
    /// widget tree.
    fn layout(&self, _: &mut Renderer) -> Box<dyn Layout>;

    /// Draw the widget to the screen.
    fn render(&self, _: &mut Renderer, _: &dyn Layout);

    fn hover(&mut self) {}
}

#[derive(Debug, Event)]
pub(crate) struct GestureDetector<E> {
    pub id: GlobalId,
    /// The message to send when the widget is hovered.
    pub hover: Option<E>,
}

impl<E: Event> GestureDetector<E> {
    pub fn new() {}
}

#[derive(Resource, Default, Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct WidgetState {
    /// True when the mouse is over the widget.
    pub hovered: bool,
    pub focused: bool,
    pub clicked: bool,
}

/// Go through all the widgets and check if they are hovered.
pub(crate) fn update_hovered_state(
    cursor_position: Res<CursorPosition>,
    layout_tree: Res<LayoutTree>,
) {
}

#[cfg(test)]
mod test {}
