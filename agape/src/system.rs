//! Systems are functions that are stored in the [`App`] and run every frame.
//! They have a `&mut` to the global [`Context`] allowing it to be modified.
//! 
//! # Create a system
//! ```
//! use agape::{hstack, App, Context};
//! use agape::system::{IntoSystem,System};
//!
//! fn current_mouse_position(cx: &mut Context){
//!     println!("Current mouse position: {}",cx.mouse_pos());
//! }
//!
//! let app = App::new(hstack! {});
//! app.add_system(current_mouse_position);
//! ```

use std::marker::PhantomData;
use crate::Context;

// EventReader<E>
// EventWriter<E>
//
//  Only run on the event
//  fn event(cx: Context, event: Event<T>){
//
//  }
//
//

/// A [`System`] is a stored procedure that has mutable access
/// to the global [`Context`] object.
pub trait System {
    fn run(&mut self, cx: &mut Context);

    fn run_on(&self) {
        // Run on a specific event
        todo!()
    }
}

/// A trait for creating systems
pub trait IntoSystem {
    type System: System;

    /// Convert a closure or function into a [`System`].
    fn into_system(self) -> Self::System;
}

impl<F: FnMut(&mut Context)> IntoSystem for F {
    type System = FunctionSystem<Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem { f: self }
    }
}

pub struct FunctionSystem<F> {
    f: F,
}

impl<F: FnMut(&mut Context)> System for FunctionSystem<F> {
    fn run(&mut self, cx: &mut Context) {
        (self.f)(cx)
    }
}

pub struct Event<T>{
    marker:PhantomData<T>,
}

pub struct EventSystem<F,E>{
    f:F,
    marker: PhantomData<E>,
}

impl<F:FnMut(&mut Context,E),E> System for EventSystem<F,E> {
    fn run(&mut self, cx: &mut Context) {
        // (self.f)(cx);
    }
}