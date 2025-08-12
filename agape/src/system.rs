//! Systems are stored procedures that run every frame.
//! They have a `&mut` to the global [`Resources`] allowing it to be modified.
use crate::Resources;
use crate::resources::{CursorPosition, EventQueue, WindowSize};
use crate::widgets::View;
use agape_core::Position;
use std::marker::PhantomData;
use winit::event::{ElementState, MouseButton, WindowEvent};

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

// TODO: make these internal, probably move them to another module

pub fn rebuild_widgets(resources: &mut Resources) {
    let view = resources.get_mut::<Box<dyn View>>().unwrap();
    view.update();
    let widget = view.view();
    resources.set(widget);
}

pub fn layout_system(resources: &mut Resources) {
    // FIXME
    let WindowSize(_) = resources.get_owned::<WindowSize>().unwrap();
}

pub fn update_cursor_position(resources: &mut Resources, event: &WindowEvent) {
    if let WindowEvent::CursorMoved { position, .. } = event {
        let cursor_position = resources.get_mut::<CursorPosition>().unwrap();
        cursor_position.0 = Position::from(*position);
    }
}

pub fn handle_mouse_button(_resources: &mut Resources, event: &WindowEvent) {
    if let &WindowEvent::MouseInput { state, button, .. } = event {
        if state != ElementState::Pressed || button != MouseButton::Left {
            return;
        }
        dbg!(&state, &button);
    }
}

pub fn handle_key_input(_: &mut Resources, event: &WindowEvent) {
    if let WindowEvent::KeyboardInput { .. } = event {}
}

pub fn intersection_observer(_: &mut Resources) {
    // FIXME
}
