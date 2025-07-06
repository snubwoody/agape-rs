//! Systems are stored procedures that run every frame.
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
