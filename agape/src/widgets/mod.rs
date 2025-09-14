//! [`Widget`]'s describe what you want to present onto the screen. Agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod button;
mod container;
mod hstack;
mod icon;
pub mod image;
mod rect;
mod svg;
mod text;
mod vstack;

use crate::LayoutTree;
use crate::assets::AssetManager;
use crate::message::{MessageQueue, MouseButtonDown};
use crate::resources::CursorPosition;
use agape_core::GlobalId;
use agape_layout::Layout;
use agape_renderer::Renderer;
use bevy_ecs::prelude::*;
pub use button::*;
pub use container::Container;
pub use hstack::*;
pub use icon::Icon;
pub use image::Image;
pub use rect::*;
use std::ops::Deref;
use std::sync::Arc;
pub use svg::Svg;
pub use text::Text;
pub use vstack::*;

#[derive(Resource)]
pub(crate) struct ViewTree(pub Box<dyn View>);
#[derive(Resource)]
pub(crate) struct WidgetTree(pub Box<dyn Widget>);

impl Deref for ViewTree {
    type Target = Box<dyn View>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for WidgetTree {
    type Target = Box<dyn Widget>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Callback = Arc<dyn Fn(&mut MessageQueue) + Send + Sync>;

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
    fn update(&mut self, _: &mut MessageQueue) {}

    fn view(&self) -> Box<dyn Widget>;
}

/// A `Widget` is anything that can ultimately be drawn to the screen. Widgets internally
/// can be composed of anything, but each widget must have a [`GlobalId`] and a [`Layout`].
pub trait Widget: Send + Sync {
    /// Get the `id` of the [`Widget`].
    fn id(&self) -> GlobalId;

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

    fn click(&mut self, _: &mut MessageQueue) {}
    fn hover(&mut self, _: &mut MessageQueue) {}
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

pub(crate) fn get_assets(assets: Res<AssetManager>, mut widget_tree: ResMut<WidgetTree>) {
    let widget = &mut widget_tree.0;
    widget.get_assets(&assets);
    widget.traverse(&mut |widget| {
        widget.get_assets(&assets);
    })
}

/// Go through all the widgets and check if they are hovered.
pub(crate) fn update_hovered_state(
    cursor_position: Res<CursorPosition>,
    layout_tree: Res<LayoutTree>,
    mut messages: ResMut<MessageQueue>,
    mut widget_tree: ResMut<WidgetTree>,
) {
    let widget = widget_tree.0.as_mut();
    let layout = layout_tree.0.as_ref();
    if let Some(l) = layout.get(widget.id())
        && cursor_position.just_hovered(l)
    {
        widget.hover(&mut messages);
    }
    widget.traverse(&mut |widget| {
        if let Some(l) = layout.get(widget.id())
            && cursor_position.just_hovered(l)
        {
            widget.hover(&mut messages);
        }
    });
}

pub(crate) fn click_widget(
    cursor_position: Res<CursorPosition>,
    layout_tree: Res<LayoutTree>,
    mut messages: ResMut<MessageQueue>,
    mut widget_tree: ResMut<WidgetTree>,
) {
    if !messages.has::<MouseButtonDown>() {
        return;
    }

    let widget = widget_tree.0.as_mut();
    let layout = layout_tree.0.as_ref();
    if let Some(l) = layout.get(widget.id())
        && cursor_position.is_hovered(l)
    {
        widget.click(&mut messages);
    }
    widget.traverse(&mut |widget| {
        if let Some(l) = layout.get(widget.id())
            && cursor_position.is_hovered(l)
        {
            widget.click(&mut messages);
        }
    });
}
