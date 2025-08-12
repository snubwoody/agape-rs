//! Systems are stored procedures that run every frame.
//! They have a `&mut` to the global [`Resources`] allowing it to be modified.

mod internal;

use crate::Resources;
use crate::resources::EventQueue;
pub(crate) use internal::*;
use std::marker::PhantomData;

/// A [`System`] is a stored procedure.
pub trait System {
    fn run(&mut self, resources: &mut Resources, event_queue: &EventQueue);
}

/// A trait for creating systems
pub trait IntoSystem<Input> {
    type System: System;

    /// Convert a closure or function into a [`System`].
    fn into_system(self) -> Self::System;
}

/// A system that runs every frame.
pub struct FunctionSystem<F> {
    f: F,
}

/// A system that runs when a specified event occurs.
pub struct EventSystem<F, E> {
    f: F,
    _marker: PhantomData<E>,
}

impl<F> IntoSystem<()> for F
where
    F: FnMut(&mut Resources),
{
    type System = FunctionSystem<Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem { f: self }
    }
}

impl<F, E> IntoSystem<(E,)> for F
where
    F: FnMut(&mut Resources, &E),
    E: 'static,
{
    type System = EventSystem<Self, E>;

    fn into_system(self) -> Self::System {
        EventSystem {
            f: self,
            _marker: PhantomData,
        }
    }
}

impl<F> System for FunctionSystem<F>
where
    F: FnMut(&mut Resources),
{
    fn run(&mut self, resources: &mut Resources, _: &EventQueue) {
        (self.f)(resources)
    }
}

impl<F, E> System for EventSystem<F, E>
where
    F: FnMut(&mut Resources, &E),
    E: 'static,
{
    fn run(&mut self, resources: &mut Resources, event_queue: &EventQueue) {
        for event in event_queue.get_all::<E>() {
            (self.f)(resources, event);
        }
    }
}
