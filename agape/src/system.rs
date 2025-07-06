//! Systems are functions that are stored in the [`App`] and run every frame.
//! They have a `&mut` to the global [`Resources`] allowing it to be modified.
//!
//! # Create a system
//! ```
//! use agape::{hstack, App, Context};
//! use agape::system::{IntoSystem, Resources, System};
//!
//! fn current_mouse_position(resources: &mut Resources){
//! }
//!
//! let app = App::new(hstack! {})
//!     .add_system(current_mouse_position);
//! ```

use agape_core::{Position, Size};
use std::any::Any;
use agape_layout::Layout;

/// A [`System`] is a stored procedure.
pub trait System {
    fn run(&mut self, resources: &mut Resources);
}

/// A trait for creating systems
pub trait IntoSystem {
    type System: System;

    /// Convert a closure or function into a [`System`].
    fn into_system(self) -> Self::System;
}

impl<F> IntoSystem for F
where
    F: FnMut(&mut Resources),
{
    type System = FunctionSystem<Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem { f: self }
    }
}

pub struct FunctionSystem<F> {
    f: F,
}

impl<F> System for FunctionSystem<F>
where
    F: FnMut(&mut Resources),
{
    fn run(&mut self, resources: &mut Resources) {
        (self.f)(resources)
    }
}

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

