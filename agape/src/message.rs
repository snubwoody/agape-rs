use crate::resources::{CursorPosition, EventQueue};
use agape_core::{GlobalId, Position};
use bevy_ecs::prelude::*;
use std::any::Any;
use winit::event::WindowEvent;

#[derive(Default, Resource, Debug)]
pub struct MessageQueue {
    items: Vec<Box<dyn Any + Send + Sync>>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.get::<T>().is_some()
    }

    pub fn add<T: 'static + Send + Sync>(&mut self, item: T) {
        // Don't insert the same resource twice
        if self.get::<T>().is_none() {
            self.items.push(Box::new(item));
        }
    }

    pub fn set<T: 'static + Send + Sync>(&mut self, item: T) {
        self.remove::<T>();
        self.items.push(Box::new(item));
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let index = self.items.iter().position(|i| i.is::<T>())?;
        let item = self.items.swap_remove(index);
        Some(*item.downcast().unwrap())
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        for item in &self.items {
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

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Message {
    MouseMoved(Position),
    MouseButtonDown,
    MouseButtonUp,
    MouseEnter(GlobalId),
    MouseLeave(GlobalId),
}

pub fn update_cursor_pos(queue: Res<EventQueue>, mut cursor_position: ResMut<CursorPosition>) {
    for event in queue.events() {
        if let WindowEvent::CursorMoved { position, .. } = event {
            cursor_position.0 = Position::from(*position);
        }
    }
}

pub fn handle_hover(queue: ResMut<EventQueue>) {}
