//! A resource is anything that needs to be accessed globally by different 
//! systems, common items such as the cursor position or the window size.
//! 
//! ### Resources
//! - [`CursorPosition`]
//! - [`WindowSize`]
//! 
use std::any::Any;
use agape_core::{Position, Size};

/// Global resources
pub struct Resources {
    items: Vec<Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Resources {
        Self { items: vec![] }
    }

    /// Insert a resource.
    pub fn insert<T: 'static>(&mut self, item: T) {
        // Don't insert the same resource twice
        if let None = self.get::<T>() {
            self.items.push(Box::new(item));
        }
    }

    /// Retrieve a resource of type `T`.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        for item in &self.items {
            match item.downcast_ref::<T>() {
                Some(item) => return Some(item),
                None => continue,
            }
        }

        None
    }

    /// Retrieve an owned resource of type `T`.
    pub fn get_owned<T: Clone + 'static>(&self) -> Option<T> {
        match self.get::<T>() {
            Some(item) => Some(item.clone()),
            None => None,
        }
    }

    /// Retrieve a mutable reference to a resource of type `T`.
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        for items in &mut self.items {
            match items.downcast_mut::<T>() {
                Some(item) => return Some(item),
                None => continue,
            }
        }

        None
    }
}

/// The current cursor position.
#[derive(Debug, Default,Copy, Clone)]
pub struct CursorPosition(pub Position);

/// The window size.
#[derive(Debug, Default,Copy, Clone)]
pub struct WindowSize(pub Size);

#[derive(Debug, Default)]
pub struct EventQueue{
    events: Vec<Box<dyn Any>>
}

impl EventQueue{
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    /// Push an event to the queue.
    pub fn push<T: 'static>(&mut self, item: T) {
        self.events.push(Box::new(item));
    }

    /// Get an event from the queue.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        for event in &self.events {
            match event.downcast_ref::<T>() {
                Some(item) => return Some(item),
                None => continue,
            }
        }

        None
    }
    
    /// Get all events of type `T` from the queue.
    pub fn get_all<T: 'static>(&self) -> Vec<&T> {
        let mut events = vec![];
        
        for event in &self.events {
            match event.downcast_ref::<T>() {
                Some(item) => events.push(item),
                None => continue,
            }
        }

        events
    }
    
    /// Clear all the events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod test{
    use super::*;
    struct DummyEvent;
    
    #[test]
    fn get_all_events(){
        let mut event_queue = EventQueue::new();
        event_queue.push(DummyEvent);
        event_queue.push(DummyEvent);
        event_queue.push(DummyEvent);
        
        let events = event_queue.get_all::<DummyEvent>();
        assert_eq!(events.len(), 3);
    }
    
    #[test]
    fn clear_events(){
        let mut event_queue = EventQueue::new();
        event_queue.push(DummyEvent);
        assert_eq!(event_queue.events.len(),1);
        
        event_queue.clear();
        assert_eq!(event_queue.events.len(),0);
    }
}