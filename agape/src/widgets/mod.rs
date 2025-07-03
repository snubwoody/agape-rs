//! [`Widget`]'s describe what you want to present onto the screen. agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
//!
//! # Custom widgets
//!
mod button;
mod hstack;
mod rect;
mod text;
mod vstack;

use crate::Context;
use crate::view::View;
pub use button::Button;
use agape_layout::Layout;
use agape_core::GlobalId;
pub use hstack::*;
pub use rect::*;
pub use text::Text;
pub use vstack::*;

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

    /// Runs every frame allowing [`Widget`]'s to manage any
    /// state they may have
    fn tick(&mut self, _cx: &Context) {}

    /// Get the widgets children.
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn click(&mut self) {}
    fn hover(&mut self) {}
}

/// The current state of the widget
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WidgetState {
    Resting,
    Hovered,
    Clicked,
}

// TODO test this
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
