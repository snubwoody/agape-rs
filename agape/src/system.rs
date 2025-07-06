//! Systems are stored procedures that run every frame.
//! They have a `&mut` to the global [`Resources`] allowing it to be modified.
//!
//! # Create a system
//! ```
//! use agape::{hstack, App, Context,Resources};
//! use agape::system::{IntoSystem, System};
//!
//! fn current_mouse_position(resources: &mut Resources){
//! }
//!
//! let app = App::new(hstack! {})
//!     .add_system(current_mouse_position);
//! ```
use crate::Resources;

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


pub struct FunctionSystem<F> {
    f: F,
}

pub struct EventSystem<F,E>{
    f: F,
    event: E
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

impl<F> System for FunctionSystem<F>
where F: FnMut(&mut Resources) {
    fn run(&mut self, resources: &mut Resources) {
        (self.f)(resources)
    }
}

impl<F,E> System for EventSystem<F,E>
where F: FnMut(&mut Resources,&E) {
    fn run(&mut self, resources: &mut Resources) {
        // get the event queue and check for e
    }    
}
