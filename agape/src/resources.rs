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

