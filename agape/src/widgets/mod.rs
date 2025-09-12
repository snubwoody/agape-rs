//! [`Widget`]'s describe what you want to present onto the screen. Agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod button;
mod container;
mod hstack;
pub mod image;
mod rect;
mod svg;
mod text;
mod vstack;

use crate::LayoutTree;
use crate::message::{MessageQueue, MouseButtonDown};
use crate::resources::CursorPosition;
use agape_core::GlobalId;
use agape_layout::Layout;
use agape_renderer::Renderer;
use bevy_ecs::prelude::*;
pub use button::*;
pub use container::Container;
pub use hstack::*;
pub use image::Image;
pub use rect::*;
use std::ops::Deref;
use std::sync::Arc;
pub use svg::Svg;
pub use text::Text;
pub use vstack::*;

#[derive(Default)]
pub struct WidgetTree2 {
    nodes: Vec<WidgetNode>,
}

impl WidgetTree2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, widget: WidgetNode) {
        self.nodes.push(widget);
    }
}

pub struct WidgetNode {
    // Parent should only be None for the root widget
    parent: Option<GlobalId>,
    children: Vec<GlobalId>,
    inner: Box<dyn Widget>,
}

impl WidgetNode {
    pub fn new<W: Widget + 'static>(inner: W) -> Self {
        Self {
            inner: Box::new(inner),
            children: Vec::new(),
            parent: None,
        }
    }

    pub fn with_parent(mut self, parent: GlobalId) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn add_child(mut self, child: GlobalId) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_children<I: IntoIterator<Item = GlobalId>>(mut self, children: I) -> Self {
        self.children.extend(children);
        self
    }

    pub fn parent(&self) -> Option<GlobalId> {
        self.parent
    }

    pub fn children(&self) -> &[GlobalId] {
        self.children.as_slice()
    }

    pub fn inner(&self) -> &dyn Widget {
        self.inner.as_ref()
    }

    pub fn inner_mut(&mut self) -> &mut dyn Widget {
        self.inner.as_mut()
    }
}

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

    fn gestures(&self) -> Option<WidgetGestures> {
        None
    }

    fn set_id(&mut self, id: GlobalId);

    fn children(&self) -> Vec<&dyn Widget>;
    fn children_mut(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {}

    /// Click the widget.
    fn click(&mut self) {}
}

#[derive(Component)]
pub struct WidgetGestures {
    pub id: GlobalId,
    pub hover: Option<Callback>,
    pub click: Option<Callback>,
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

/// Go through all the widgets and check if they are hovered.
pub(crate) fn update_hovered_state(
    cursor_position: Res<CursorPosition>,
    layout_tree: Res<LayoutTree>,
    mut messages: ResMut<MessageQueue>,
    query: Query<&WidgetGestures>,
) {
    let layout = layout_tree.0.as_ref();
    for gesture in query.iter() {
        if let Some(l) = layout.get(gesture.id)
            && cursor_position.just_hovered(l)
            && let Some(hover) = &gesture.hover
        {
            hover(&mut messages);
        }
    }
}

pub(crate) fn click_widget(
    cursor_position: Res<CursorPosition>,
    layout_tree: Res<LayoutTree>,
    mut messages: ResMut<MessageQueue>,
    query: Query<&WidgetGestures>,
    mut widget_tree: ResMut<WidgetTree>,
) {
    if !messages.has::<MouseButtonDown>() {
        return;
    }

    let widget = widget_tree.0.as_mut();
    widget.click();
    widget.traverse(&mut |widget| {
        messages.has::<MouseButtonDown>();
        widget.click()
    });
    let layout = layout_tree.0.as_ref();
    for gesture in query.iter() {
        if let Some(l) = layout.get(gesture.id)
            && cursor_position.is_hovered(l)
            && let Some(click) = &gesture.click
        {
            click(&mut messages);
        }
    }
}

pub(crate) fn spawn_widget_gestures(
    mut commands: Commands,
    widget: Res<WidgetTree>,
    query: Query<Entity, With<WidgetGestures>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    let gestures: Vec<WidgetGestures> = widget.iter().filter_map(|w| w.gestures()).collect();
    for gesture in gestures {
        commands.spawn(gesture);
    }
}
