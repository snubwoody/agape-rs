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
    fn update(&mut self, _: &mut MessageQueue) {}

    fn view(&self) -> Box<dyn Widget>;
}

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
}

#[derive(Default, Resource, Debug, PartialEq)]
pub struct StateTracker {
    previous: HashMap<GlobalId, WidgetState>,
    state: HashMap<GlobalId, WidgetState>,
}

impl StateTracker {
    pub fn from_layout(layout_tree: &dyn Layout) -> Self {
        let mut state = HashMap::new();
        for layout in layout_tree.iter() {
            state.insert(layout.id(), WidgetState::default());
        }
        let previous = state.clone();
        Self { state, previous }
    }

    fn check_hovered(&mut self, layout_tree: &dyn Layout, cursor_pos: &CursorPosition) {
        // TODO: Test this
        self.previous = self.state.clone();
        for (id, state) in self.state.iter_mut() {
            let layout = layout_tree.get(*id).unwrap();
            if layout.bounds().within(cursor_pos) {
                state.hovered = true
            }
        }
    }

    fn just_hovered(&self, id: GlobalId) {}
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
    mut state_tracker: ResMut<StateTracker>,
    cursor_position: Res<CursorPosition>,
    layout_tree: Res<LayoutTree>,
) {
    state_tracker.check_hovered(&*layout_tree.0, &cursor_position);
    dbg!(&state_tracker);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vstack;
    use agape_core::{Position, Size};
    use agape_layout::solve_layout;

    #[test]
    fn check_hovered() {
        let widget = vstack! {}.fill();
        let mut layout = widget.layout(&mut Renderer::new());
        solve_layout(layout.as_mut(), Size::unit(200.0));
        let cursor_pos = CursorPosition(Position::unit(100.0));
        let mut state = StateTracker::from_layout(layout.as_ref());
        state.check_hovered(layout.as_ref(), &cursor_pos);
        assert!(state.state[&widget.id()].hovered);
    }

    #[test]
    fn update_previous_hovered() {
        let widget = vstack! {}.fill();
        let mut layout = widget.layout(&mut Renderer::new());
        solve_layout(layout.as_mut(), Size::unit(200.0));
        let cursor_pos = CursorPosition(Position::unit(100.0));
        let mut state = StateTracker::from_layout(layout.as_ref());
        state.check_hovered(layout.as_ref(), &cursor_pos);
        assert!(!state.previous[&widget.id()].hovered);
        state.check_hovered(layout.as_ref(), &cursor_pos);
        assert!(state.previous[&widget.id()].hovered);
    }
}
