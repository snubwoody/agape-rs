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
    fn tick(&mut self) {}

    fn process_key(&mut self, key: &winit::keyboard::Key) {}

    fn click(&mut self) {}

    /// Respond to the user scrolling, triggered by either the touchpad or mousewheel.
    fn scroll(&mut self, delta: Position) {}

    /// Set the [`Widget`]'s focus state to false when the cursor clicks outside
    /// the widget's bounds.
    fn unfocus(&mut self) {}

    // TODO maybe make a test macro to make sure all widgets
    // handle this right
    /// Get the direct children of the [`Widget`]
    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        &mut []
    }
}

impl dyn Widget {
    pub(crate) fn handle_event(&mut self, event: &WindowEvent) {
        
    }
    
    pub fn update(&mut self) {
        self.tick();
        for child in self.children_mut() {
            child.tick();
        }
    }

    /// Handles `winit`'s click event.
    fn dispatch_click(
        &mut self,
        state: &winit::event::ElementState,
        button: &winit::event::MouseButton,
    ) {
        if button == &MouseButton::Left && state == &ElementState::Pressed {
            self.click();
        }
    }

    /// Handles all `winit` events
    pub(crate) fn dispatch_event(
        &mut self,
        mouse_pos: helium_core::Position,
        layout_tree: &dyn Layout,
        window_event: &WindowEvent,
    ) {
        // I feel like events might happen out of order because of winit's event loop but we shall find out
        if let Some(layout) = layout_tree.get(self.id()) {
            match window_event {
                WindowEvent::KeyboardInput { event, .. } => match event.state {
                    ElementState::Pressed => {
                        self.process_key(&event.logical_key);
                    }
                    ElementState::Released => {}
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    let bounds = Bounds::new(layout.position(), layout.size());

                    if bounds.within(&mouse_pos) {
                        self.dispatch_click(state, button)
                    } else {
                        self.unfocus();
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let position = match delta {
                        MouseScrollDelta::LineDelta(x, y) => Position::new(*x, *y),
                        MouseScrollDelta::PixelDelta(pos) => {
                            Position::new(pos.x as f32, pos.y as f32)
                        }
                    };
                    // TODO check for mouse position
                    self.scroll(position);
                }
                _ => {}
            }
        } else {
            log::warn!("Widget: {} is missing a Layout", self.id())
        }

        for child in self.children_mut() {
            child.dispatch_event(mouse_pos, layout_tree, window_event);
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

