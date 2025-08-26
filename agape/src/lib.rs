//! A cross-platform GUI library.
//!
//! ## Getting started
//! To get started you'll need to create an [`App`], which is the entry point
//! of the program, and a root [`Widget`].
use crate::widgets::Widget;
pub mod error;
mod macros;
pub mod message;
pub mod resources;
pub mod style;
pub mod widgets;

pub use agape_core::*;
pub use agape_layout as layout;
pub use agape_macros::hex;
pub use agape_renderer as renderer;
pub use error::{Error, Result};
pub use message::MessageQueue;
use message::update_cursor_pos;
use resources::CursorPosition;
use resources::EventQueue;
pub use resources::Resources;
use widgets::View;

use crate::message::{MouseButtonDown, MouseButtonUp};
use agape_layout::{Layout, solve_layout};
use agape_renderer::Renderer;
use bevy_ecs::prelude::*;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::event::{ElementState, MouseButton};
use winit::event_loop::ActiveEventLoop;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowId,
};

// TODO: store the pixmap in the renderer?
/// An `App` is a single program.
pub struct App<'app, V> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    renderer: Renderer,
    view: V,
    state: State,
    world: World,
    schedule: Schedule,
}

impl<V: View> App<'_, V> {
    /// Create a new app.
    pub fn new(view: V) -> Self {
        let widget = view.view();
        let mut renderer = Renderer::new();
        let layout = widget.layout(&mut renderer);
        let state = State::new(layout);

        let mut world = World::new();
        world.insert_resource(CursorPosition::default());
        world.insert_resource(EventQueue::default());
        world.insert_resource(MessageQueue::default());

        Self {
            window: None,
            pixels: None,
            renderer,
            view,
            state,
            world,
            schedule: Schedule::default(),
        }
    }

    fn render(&mut self) {
        let mut messages = self.world.get_resource_mut::<MessageQueue>().unwrap();
        // TODO: move tick and clear to system
        // TODO: update_view
        messages.tick();
        self.view.update(&self.state, messages.as_mut());
        messages.clear();

        // TODO: update_layout
        let widget = self.view.view();
        let mut layout = widget.layout(&mut self.renderer);
        solve_layout(&mut *layout, self.state.window_size);

        let pixels = self.pixels.as_mut().unwrap();
        self.renderer.pixmap_mut().fill(tiny_skia::Color::WHITE);

        widget.render(&mut self.renderer, layout.as_ref());

        self.state.update_layout(layout);
        pixels
            .frame_mut()
            .copy_from_slice(self.renderer.pixmap().data());
        pixels.render().unwrap();
    }

    /// Run the app.
    ///
    /// # Panics
    /// The app will panic if it is run in another thread, this is
    /// because accessing windows in other threads is unsafe on
    /// certain platforms.
    pub fn run(mut self) -> Result<()> {
        self.schedule
            .add_systems(update_cursor_pos)
            .add_systems(handle_click)
            .add_systems(clear_events);
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl<V: View> ApplicationHandler for App<'_, V> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Initializing resources");
        let window = event_loop.create_window(Default::default()).unwrap();
        let window = Arc::new(window);

        let size = Size::from(window.inner_size());
        let width = size.width as u32;
        let height = size.height as u32;

        let surface = SurfaceTexture::new(width, height, Arc::clone(&window));
        let pixels = Pixels::new(width, height, surface).unwrap();

        self.pixels = Some(pixels);
        self.window = Some(Arc::clone(&window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.schedule.run(&mut self.world);
        self.world
            .get_resource_mut::<EventQueue>()
            .unwrap()
            .push(event.clone());
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Exiting app");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.as_mut().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.pixels
                    .as_mut()
                    .unwrap()
                    .resize_surface(size.width, size.height)
                    .expect("Failed to resize the pixel buffer");

                self.pixels
                    .as_mut()
                    .unwrap()
                    .resize_buffer(size.width, size.height)
                    .expect("Failed to resize the pixel buffer");

                self.renderer.resize(size.width, size.height);
                self.state.window_size = Size::from(size);
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos: Position = position.into();
                self.state.update_cursor_pos(pos);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct State {
    cursor_position: Position,
    window_size: Size,
    layout: Box<dyn Layout>,
}

impl State {
    pub fn new(layout: Box<dyn Layout>) -> Self {
        Self {
            cursor_position: Position::default(),
            window_size: Size::default(),
            layout,
        }
    }

    /// Check if the cursor is over the [`Widget`].
    pub fn is_hovered(&self, id: &GlobalId) -> bool {
        if let Some(layout) = self.layout.get(*id) {
            return layout.bounds().within(&self.cursor_position);
        }

        false
    }

    pub fn update_cursor_pos(&mut self, pos: Position) {
        self.cursor_position = pos;
    }

    pub fn update_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }
}

fn handle_click(queue: ResMut<EventQueue>, mut messages: ResMut<MessageQueue>) {
    for event in queue.events() {
        if let WindowEvent::MouseInput { button, state, .. } = event {
            if let MouseButton::Left = button
                && let ElementState::Pressed = state
            {
                messages.add(MouseButtonDown);
            }

            if let MouseButton::Left = button
                && let ElementState::Released = state
            {
                messages.add(MouseButtonUp);
            }
        }
    }
}

fn clear_events(mut queue: ResMut<EventQueue>) {
    queue.clear();
    queue.tick();
}
