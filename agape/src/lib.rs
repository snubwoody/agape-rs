//! A cross-platform GUI library.
//!
//! ## Getting started
//! To get started you'll need to create an [`App`], which is the entry point
//! of the program, and a root [`Widget`].
pub mod error;
mod macros;
pub mod resources;
pub mod style;
pub mod widgets;

pub use agape_core::*;
pub use agape_layout as layout;
pub use agape_macros::hex;
pub use agape_renderer::Renderer;
pub use error::{Error, Result};
pub use resources::Resources;
use resources::{CursorPosition, EventQueue, WindowSize};

use crate::widgets::View;
use agape_layout::{Layout, solve_layout};
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use tiny_skia::Pixmap;
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
pub struct App<'app> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    pixmap: Option<Pixmap>,
    renderer: Renderer,
    view: Box<dyn View>,
    window_size: Size,
    messages: Vec<Message>,
    state: State,
}

impl App<'_> {
    /// Create a new app.
    pub fn new(root: impl View + 'static) -> Self {
        let widget = root.view();
        let mut renderer = Renderer::new();
        let queue: Vec<Message> = Vec::new();
        let view: Box<dyn View> = Box::new(root);
        let layout = widget.layout(&mut renderer);
        let state = State::new(layout);
        // TODO: test these
        let mut resources = Resources::new();
        resources.insert(CursorPosition::default());
        resources.insert(WindowSize::default());
        resources.insert(widget);
        resources.insert(queue);

        Self {
            window: None,
            pixmap: None,
            pixels: None,
            window_size: Size::default(),
            messages: Vec::new(),
            renderer,
            view,
            state,
        }
    }

    fn render(&mut self) {
        for message in self.messages.drain(..) {
            self.view.update(&message, &self.state);
        }
        // This is very much a hack
        let widget = self.view.view();
        let mut layout = widget.layout(&mut self.renderer);
        solve_layout(&mut *layout, self.window_size);

        let pixels = self.pixels.as_mut().unwrap();
        let pixmap = self.pixmap.as_mut().unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        widget.render(pixmap, &mut self.renderer, layout.as_ref());

        self.state.update_layout(layout);
        pixels.frame_mut().copy_from_slice(pixmap.data());
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
        log::info!("Initializing resources");
        let window = event_loop.create_window(Default::default()).unwrap();
        let window = Arc::new(window);

        let size = Size::from(window.inner_size());
        let width = size.width as u32;
        let height = size.height as u32;

        let surface = SurfaceTexture::new(width, height, Arc::clone(&window));
        let pixels = Pixels::new(width, height, surface).unwrap();
        let pixmap = Pixmap::new(width, height).unwrap();

        self.pixels = Some(pixels);
        self.window = Some(Arc::clone(&window));
        self.pixmap = Some(pixmap);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
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

                let pixmap = Pixmap::new(size.width, size.height).unwrap();
                self.pixmap = Some(pixmap);
                self.window_size = Size::from(size);
            }
            WindowEvent::CursorMoved { position, .. } => {
                let pos: Position = position.into();
                self.state.update_cursor_pos(pos);
                self.messages.push(Message::MouseMoved(pos))
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct State {
    cursor_position: Position,
    layout: Box<dyn Layout>,
}

impl State {
    pub fn new(layout: Box<dyn Layout>) -> Self {
        Self {
            cursor_position: Position::default(),
            layout,
        }
    }

    /// Check if the cursor is over the [`Widget`].
    ///
    /// # Panics
    /// Panics if the widget is not found.
    pub fn is_hovered(&self, id: &GlobalId) -> bool {
        self.layout
            .get(*id)
            .unwrap()
            .bounds()
            .within(&self.cursor_position)
    }

    pub fn update_cursor_pos(&mut self, pos: Position) {
        self.cursor_position = pos;
    }

    pub fn update_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = layout;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Message {
    MouseMoved(Position),
}
