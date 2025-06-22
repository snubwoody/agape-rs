//! [`Widget`]'s describe what you want to present onto the screen. Helium tries to provide
//! as many [`Widget`]'s as possible for various uses such as [`Text`],[`Button`],[`HStack`]
//! and [`VStack`], and the list goes on. Every widget must implement the [`Widget`] trait.
mod hstack;
mod rect;
mod text;
mod vstack;

use crate::view::{RectView, View};
use crystal::Layout;
use helium_core::{Bounds, Color, GlobalId, Position, Rgba};
pub use hstack::*;
pub use rect::*;
pub use text::Text;
pub use vstack::*;
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};

pub trait Widget: WidgetIterator {
    fn view(&self) -> Box<dyn View> {
        Box::new(RectView::new(Color::BLACK))
    }

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
    fn tick(&mut self) {
        todo!()
    }

    // TODO maybe make a test macro to make sure all widgets
    // handle this right
    /// Get the direct children of the [`Widget`]
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }
    
    fn handle_text_input(&mut self, text: &str){
        
    }
}

impl dyn Widget {
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event { 
            &WindowEvent::KeyboardInput {ref event,..} => {
                if let Some(text) = &event.text{
                    self.handle_text_input(text);
                }
            },
            _ => (),
        }   
        
        for child in self.children_mut(){
            child.handle_event(event);
        }
    }
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

