use crate::resources::{CursorPosition, EventQueue};
use agape_core::Position;
use bevy_ecs::prelude::*;
use std::any::Any;
use winit::event::WindowEvent;

/// Emitted when the left mouse button is pressed.
pub struct MouseButtonDown;

/// Emitted when the left mouse button is pressed.
pub struct MouseButtonUp;

// Marker trait
/// The message trait is implemented for anything which implements
/// `Any + Send + Sync`.
pub trait Message: Any + Send + Sync {}

impl<T: Any + Send + Sync> Message for T {}

#[derive(Default, Resource)]
pub struct MessageQueue {
    items: Vec<Box<dyn Message>>,
    frame_count: u32,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
    }

    pub fn has<M: Message>(&self) -> bool {
        self.get::<M>().is_some()
    }

    pub fn add<M: Message>(&mut self, item: M) {
        // Don't insert the same resource twice
        if self.get::<M>().is_none() {
            self.items.push(Box::new(item));
        }
    }

    pub fn set<M: Message>(&mut self, item: M) {
        self.remove::<M>();
        self.items.push(Box::new(item));
    }

    /// Remove and return a message of type `M` from the queue.
    pub fn remove<M: 'static>(&mut self) -> Option<M> {
        let index = self
            .items
            .iter()
            .map(|i| i.as_ref() as &dyn Any)
            .position(|i| i.is::<M>())?;
        let item: Box<dyn Any> = self.items.swap_remove(index);
        item.downcast().ok().map(|m| *m)
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        for item in &self.items {
            let item = item.as_ref() as &dyn Any;
            match item.downcast_ref::<T>() {
                Some(item) => return Some(item),
                None => continue,
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        // TODO: 2 frames might be better
        if self.frame_count >= 3 {
            self.items.clear();
            self.frame_count = 0;
        }
    }
}

pub fn update_cursor_pos(queue: Res<EventQueue>, mut cursor_position: ResMut<CursorPosition>) {
    for event in queue.events() {
        if let WindowEvent::CursorMoved { position, .. } = event {
            cursor_position.update(Position::from(*position));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn message_queue_tick() {
        let mut messages = MessageQueue::new();
        messages.tick();
        messages.tick();
        assert_eq!(messages.frame_count, 2);
    }

    #[test]
    fn clear_messages() {
        let mut messages = MessageQueue::new();
        messages.add(String::new());
        messages.tick();
        messages.tick();
        messages.clear();
        assert_eq!(messages.len(), 1);
        messages.tick();
        messages.clear();
        assert!(messages.is_empty());
    }
}
