//! A resource is anything that needs to be accessed globally by different
//! systems, common items such as the cursor position or the window size.
//!
//! Resources work on types, `T`, so avoid setting primitive or commonly used types
//! like `String` or `Box` as it will make tracking things much harder.
use agape_core::{Position, Size};
use bevy_ecs::prelude::Resource;
use std::any::Any;
use std::ops::Deref;
use winit::event::WindowEvent;

/// Global resources
#[derive(Default)]
pub struct Resources {
    items: Vec<Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Resources {
        Self { items: vec![] }
    }

    /// Inserts a resource.
    ///
    /// To insert or replace a resource use [`set`].
    ///
    /// [`set`]: Resources::set
    pub fn insert<T: 'static>(&mut self, item: T) {
        // Don't insert the same resource twice
        if self.get::<T>().is_none() {
            self.items.push(Box::new(item));
        }
    }

    /// Inserts or replaces a resource.
    pub fn set<T: 'static>(&mut self, item: T) {
        self.remove::<T>();
        self.items.push(Box::new(item));
    }

    /// Removes the resource and returns it.
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let index = self.items.iter().position(|i| i.is::<T>())?;
        let item = self.items.swap_remove(index);
        Some(*item.downcast().unwrap())
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
        self.get::<T>().cloned()
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

    /// Returns the number of resources.
    ///
    /// # Example
    /// ```
    /// use agape::{Resources,Position,Size};
    ///
    /// let mut resources = Resources::new();
    /// resources.insert(Position::new(0.0,0.0));
    /// resources.insert(Size::new(50.0,0.0));
    ///
    /// assert_eq!(resources.len(),2);
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// The current cursor position.
#[derive(Debug, Default, Copy, Clone, Resource)]
pub struct CursorPosition(pub Position);

impl Deref for CursorPosition {
    type Target = Position;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The window size.
#[derive(Debug, Default, Copy, Clone)]
pub struct WindowSize(pub Size);

#[derive(Debug, Default, Resource)]
pub struct EventQueue {
    events: Vec<WindowEvent>,
    frame_count: u32,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an event to the queue.
    pub fn push(&mut self, event: WindowEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[WindowEvent] {
        self.events.as_slice()
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
    }

    pub fn clear(&mut self) {
        if self.frame_count > 3 {
            self.events.clear();
            self.frame_count = 0;
        }
    }
}
