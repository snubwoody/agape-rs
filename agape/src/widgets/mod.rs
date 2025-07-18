//! [`Widget`]'s describe what you want to present onto the screen. Agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
//!
//! # Creating custom widgets
//! To create a custom widget you can implement the `Widget` trait, it has three required
//! methods:
//! - `id`: Return the widgets [`GlobalId`].
//! - `view`: Return the widgets [`View`] for rendering.
//! - `layout`: Return the widgets [`Layout`] for layout calculations.
//!
//! Additionally, if your widget has any children you will need to implement the `children`
//! method.
mod button;
mod hstack;
mod rect;
mod text;
mod text_field;
mod vstack;

use crate::view::View;
use agape_core::GlobalId;
use agape_layout::{IntrinsicSize, Layout};
pub use button::Button;
pub use hstack::*;
pub use rect::*;
use std::collections::HashMap;
pub use text::Text;
pub use text_field::TextField;
pub use vstack::*;
use winit::event::ElementState;
use winit::keyboard;

pub trait Widget: WidgetIterator {
    fn view(&self) -> Box<dyn View>;

    /// Get the widget's [`Layout`]
    fn layout(&self) -> Box<dyn Layout>;

    /// Get the `id` of the [`Widget`]
    fn id(&self) -> GlobalId;

    /// Get a [`Widget`] from the widget tree by it's `id`
    fn get(&self, id: GlobalId) -> Option<&dyn Widget> {
        self.iter().find(|&widget| widget.id() == id)
    }

    fn traverse(&self, _f: &mut dyn FnMut(&dyn Widget)) {}
    fn traverse_mut(&mut self, _f: &mut dyn FnMut(&mut dyn Widget)) {}

    /// Get the widgets children.
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }

    fn handle_event(&mut self, event: &WidgetEvent) {
        match event {
            WidgetEvent::Hovered(id) => {
                if id == &self.id() {
                    self.hover();
                }
            }
            WidgetEvent::Clicked(id) => {
                if id == &self.id() {
                    self.click();
                }
            }
            WidgetEvent::KeyInput { key, state, text } => {
                self.key_input(key, state, text);
            }
        }

        self.traverse_mut(&mut |child| child.handle_event(event));
    }

    fn click(&mut self) {}
    fn hover(&mut self) {}

    fn key_input(&mut self, _key: &keyboard::Key, _state: &ElementState, _text: &Option<String>) {}
}

#[derive(Clone, PartialEq, Debug)]
pub enum WidgetEvent {
    Hovered(GlobalId),
    Clicked(GlobalId),
    KeyInput {
        key: keyboard::Key,
        state: ElementState,
        text: Option<String>,
    },
}

pub enum LayoutDescription {
    EmptyLayout {
        id: GlobalId,
        intrinsic_size: IntrinsicSize,
    },
    BlockLayout {
        id: GlobalId,
        intrinsic_size: IntrinsicSize,
        padding: u32,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub struct StateTracker {
    previous_state: HashMap<GlobalId, WidgetState>,
    current_state: HashMap<GlobalId, WidgetState>,
}

// TODO test these
impl StateTracker {
    pub fn new(widget: &dyn Widget) -> Self {
        let mut previous_state = HashMap::new();
        let mut current_state = HashMap::new();

        widget.iter().for_each(|w| {
            let id = w.id();
            previous_state.insert(id, WidgetState::Resting);
            current_state.insert(id, WidgetState::Resting);
        });

        Self {
            previous_state,
            current_state,
        }
    }

    pub fn current_state(&self, id: GlobalId) -> Option<&WidgetState> {
        self.current_state.get(&id)
    }

    pub fn previous_state(&self, id: GlobalId) -> Option<&WidgetState> {
        self.previous_state.get(&id)
    }

    pub fn update_state(&mut self, id: GlobalId, state: WidgetState) {
        let previous_state = self.current_state.get(&id).unwrap();
        self.previous_state.insert(id, *previous_state);
        self.current_state.insert(id, state);
    }
}

/// The current state of the widget
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WidgetState {
    Resting,
    Hovered,
    Clicked,
}

/// An iterator over the [`Widget`] tree.
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

pub trait WidgetIterator {
    fn iter(&self) -> WidgetIter<'_>;
}

impl<T: Widget> WidgetIterator for T {
    fn iter(&self) -> WidgetIter<'_> {
        WidgetIter { stack: vec![self] }
    }
}
