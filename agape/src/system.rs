//! Systems are stored procedures that run every frame.
//! They have a `&mut` to the global [`Resources`] allowing it to be modified.
//!
//! # Create a system
//! ```
//! use agape::{hstack, App, Resources};
//! use agape::system::{IntoSystem, System};
//!
//! fn say_hello(resources: &mut Resources){
//!     println!("Hi there!");
//! }
//!
//! let app = App::new(hstack! {})
//!     .add_system(say_hello);
//! ```
//! ## Event systems
//! An [`EventSystem`] is a system that only runs when a specific event is emitted. You
//! can create an event system by adding the event as a parameter.
//!
//! ```
//! use winit::event::WindowEvent;
//! use agape::{hstack, App, Resources};
//! use agape::system::{System,IntoSystem};
//!
//! fn window_event(res: &mut Resources,event: &WindowEvent){
//!     println!("New event: {:#?}",event);
//! }
//!
//! let app = App::new(hstack! {})
//!     .add_system(window_event);
//! ```

use crate::Resources;
use crate::resources::{CursorPosition, EventQueue, WindowSize};
use crate::widgets::{RenderBox, StateTracker, Widget, WidgetEvent, WidgetState};
use agape_core::{Bounds, Position};
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

pub fn update_widgets(resources: &mut Resources) {
    let state = resources.get_owned::<StateTracker>().unwrap();
    let widget = resources.get_mut::<Box<dyn Widget>>().unwrap();
    widget.tick(&state);
}

pub fn rebuild_widgets(resources: &mut Resources) {
    let widget = resources.get::<Box<dyn Widget>>().unwrap();
    // FIXME: rebuild widgets
    // let render_box = widget.build();
    // resources.set(render_box);
}

pub fn layout_system(resources: &mut Resources) {
    let WindowSize(size) = resources.get_owned::<WindowSize>().unwrap();

    let render_box = resources.get_mut::<RenderBox>().unwrap();
    render_box.solve_layout(size);
}

pub fn update_cursor_position(resources: &mut Resources, event: &WindowEvent) {
    if let WindowEvent::CursorMoved { position, .. } = event {
        let cursor_position = resources.get_mut::<CursorPosition>().unwrap();
        cursor_position.0 = Position::from(*position);
    }
}

pub fn handle_mouse_button(resources: &mut Resources, event: &WindowEvent) {
    match event {
        &WindowEvent::MouseInput { state, button, .. } => {
            if state != ElementState::Pressed || button != MouseButton::Left {
                return;
            }
        }
        _ => return,
    }

    let cursor_position: &CursorPosition = resources.get().unwrap();

    let render_box = resources.get::<RenderBox>().unwrap();
    let mut hovered = vec![];

    for rb in render_box.iter() {
        let bounds = Bounds::new(rb.position, rb.size);
        if bounds.within(&cursor_position.0) {
            hovered.push(rb.id());
        }
    }

    let state_tracker = resources.get_mut::<StateTracker>().unwrap();
    for id in &hovered {
        state_tracker.update_state(*id, WidgetState::Clicked);
    }

    let event_queue = resources.get_mut::<Vec<WidgetEvent>>().unwrap();
    for id in hovered {
        event_queue.push(WidgetEvent::Clicked(id));
    }
}

pub fn handle_key_input(resources: &mut Resources, event: &WindowEvent) {
    if let WindowEvent::KeyboardInput { event, .. } = event {
        let events = resources.get_mut::<Vec<WidgetEvent>>().unwrap();
        let widget_event = WidgetEvent::KeyInput {
            key: event.logical_key.clone(),
            state: event.state,
            text: event.text.clone().map(|t| t.to_string()),
        };

        events.push(widget_event);
    }
}

pub fn intersection_observer(resources: &mut Resources) {
    let cursor_pos = resources.get::<CursorPosition>().unwrap();
    let render_box = resources.get::<RenderBox>().unwrap();
    let mut hovered = vec![];
    let mut not_hovered = vec![];

    for rb in render_box.iter() {
        let bounds = Bounds::new(rb.position, rb.size);
        if bounds.within(&cursor_pos.0) {
            hovered.push(rb.id());
        } else {
            not_hovered.push(rb.id());
        }
    }

    let state = resources.get_mut::<StateTracker>().unwrap();
    for id in &hovered {
        state.update_state(*id, WidgetState::Hovered);
    }

    for id in &not_hovered {
        state.update_state(*id, WidgetState::Resting);
    }

    let state = resources.get::<StateTracker>().unwrap();
    let mut events = vec![];
    for id in &hovered {
        if state.previous_state(*id).unwrap() == &WidgetState::Resting {
            events.push(WidgetEvent::Hovered(*id));
        }
    }

    let widget_events: &mut Vec<WidgetEvent> = resources.get_mut().unwrap();
    widget_events.extend(events);
}

pub fn handle_widget_event(resources: &mut Resources) {
    let events: Vec<WidgetEvent> = resources.get_owned().unwrap();
    let widget: &mut Box<dyn Widget> = resources.get_mut().unwrap();

    for event in events {
        widget.handle_event(&event);
    }

    resources.get_mut::<Vec<WidgetEvent>>().unwrap().clear();
}
