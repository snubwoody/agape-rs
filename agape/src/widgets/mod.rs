//! [`Widget`]'s describe what you want to present onto the screen. agape tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod button;
mod hstack;
mod rect;
mod text;
mod vstack;

use crate::view::View;
use agape_core::GlobalId;
use agape_layout::Layout;
pub use button::Button;
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
    
    /// Get the widgets children.
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }
    
    fn children_mut(&mut self) -> Vec<&mut dyn Widget> {
        vec![]
    }

    fn click(&mut self) {}
    fn hover(&mut self) {}
}

pub enum WidgetEvent{
    Hovered(GlobalId),
    Clicked(GlobalId),
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
