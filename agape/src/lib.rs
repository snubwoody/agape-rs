//! Agape is a cross-platform GUI library.
//!
//! ## Overview
//! At the core of `agape` is widgets, a [`Widget`] is anything that holds state and can be drawn
//! to the screen. To get started create an [`App`] with a root widget.
//!
//! ```no_run
//! use agape::{App,widgets::*,Error};
//! fn main() -> Result<(),Error>{
//!     App::new(Home())
//!         .run()
//! }
//!
//! fn Home() -> impl Widget{
//!     Text::new("Hello!!")
//!         .font_size(24)
//! }
//! ```
mod assets;
pub mod error;
mod macros;
pub mod message;
pub mod resources;
mod state;
pub mod style;
pub mod widgets;

pub use agape_core::*;
pub use agape_layout as layout;
pub use agape_macros::hex;
pub use agape_renderer as renderer;
pub use error::{Error, Result};
pub use message::{Message, MessageQueue};
use std::path::Path;

use crate::message::{MouseButtonDown, MouseButtonUp};
use crate::state::{Scroll, State};
use crate::widgets::Widget;
pub use agape_macros::Widget;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use tracing::info;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowId,
};

/// An `App` is a single program.
pub struct App<'app> {
    // The window and pixel buffer only get populated
    // when the window actually opens.
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    state: State,
}

impl App<'_> {
    /// Create a new app.
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            state: State::new(widget),
            pixels: None,
            window: None,
        }
    }

    pub fn assets(mut self, path: impl AsRef<Path>) -> Self {
        self.state.asset_dir(path);
        self
    }

    fn render(&mut self) {
        self.state.render();
        let renderer = self.state.renderer();
        let pixels = self.pixels.as_mut().unwrap();

        pixels.frame_mut().copy_from_slice(renderer.pixmap().data());
        pixels.render().unwrap();
    }

    /// Run the app.
    ///
    /// # Panics
    /// The app will panic if it is run in another thread, this is
    /// because accessing windows in other threads is unsafe on
    /// certain platforms.
    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Initializing resources");
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
        match event {
            WindowEvent::CloseRequested => {
                info!("Exiting app");
                event_loop.exit();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.state.update_cursor_position(position.into());
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let messages = self.state.messages_mut();
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

                self.state.resize(size.into());
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.state.key_event(&event);
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_, y),
                ..
            } => {
                self.state.messages_mut().add(Scroll(y));
            }
            _ => {}
        }
        self.state.update();
    }
}
