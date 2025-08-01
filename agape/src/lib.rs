//! A cross-platform GUI library.
//!
//! ## Getting started
//! To get started you'll need to create an [`App`], which is the entry point
//! of the program, and a root [`Widget`].
//!
//! ```no_run
//! use agape::{App,hstack,widgets::Text};
//!
//! let hstack = hstack! {
//!     Text::new("Hello"),
//!     Text::new("world")
//! }
//! .padding(12)
//! .spacing(12)
//! .align_center()
//! .fill();
//!
//! let app = App::new(hstack);
//! app.run().unwrap();
//! ```
pub mod error;
mod macros;
mod renderer;
pub mod resources;
pub mod style;
pub mod system;
pub mod widgets;

pub use agape_core::*;
pub use agape_layout as layout;
pub use agape_macros::hex;
pub use error::{Error, Result};
use renderer::init_font;
pub use resources::Resources;
use resources::{CursorPosition, EventQueue, WindowSize};
use system::{IntoSystem, System, *};
use widgets::Widget;
use widgets::{RenderBox, StateTracker, WidgetEvent};

use fontdue::Font;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use std::sync::OnceLock;
use tiny_skia::Pixmap;
use winit::event_loop::ActiveEventLoop;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowId,
};

static FONT: OnceLock<Font> = OnceLock::new();

/// An `App` is a single program.
pub struct App<'app> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    pixmap: Option<Pixmap>,
    resources: Resources,
    event_queue: EventQueue,
    systems: Vec<Box<dyn System>>,
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
        self.event_queue.push(event.clone());

        for system in self.systems.iter_mut() {
            system.run(&mut self.resources, &self.event_queue);
        }

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
                self.resources.get_mut::<WindowSize>().unwrap().0 = Size::from(size);
            }
            WindowEvent::MouseInput { .. } => {}
            _ => {}
        }

        self.event_queue.clear();
    }
}

impl App<'_> {
    /// Create a new app.
    pub fn new(widget: impl Widget + 'static) -> Self {
        FONT.set(init_font()).unwrap();

        let widget: Box<dyn Widget> = Box::new(widget);
        let render_box = widget.build();
        let state_tracker = StateTracker::new(&render_box);

        let mut resources = Resources::new();
        resources.insert(state_tracker);
        resources.insert(render_box);
        resources.insert(CursorPosition::default());
        resources.insert(WindowSize::default());
        resources.insert(EventQueue::new());
        resources.insert(widget);
        resources.insert::<Vec<WidgetEvent>>(Vec::new());

        let systems = vec![Box::new(layout_system.into_system()) as Box<dyn System>];

        Self {
            event_queue: EventQueue::new(),
            window: None,
            pixmap: None,
            pixels: None,
            resources,
            systems,
        }
    }

    /// Add a [`System`].
    ///
    /// # Example
    /// ```
    /// use agape::{hstack, App};
    /// use agape::resources::{CursorPosition, Resources};
    ///
    /// fn cursor_position(resources: &mut Resources){
    ///     let position = resources.get::<CursorPosition>().unwrap();
    ///     println!("Current position: {:?}",position);
    /// }
    ///
    /// let app = App::new(hstack!{})
    ///     .add_system(cursor_position);
    /// ```
    pub fn add_system<Input: 'static>(mut self, f: impl IntoSystem<Input> + 'static) -> Self {
        self.systems.push(Box::new(f.into_system()));
        self
    }

    fn render(&mut self) {
        let render_box = self.resources.get::<RenderBox>().unwrap();

        let pixels = self.pixels.as_mut().unwrap();
        let pixmap = self.pixmap.as_mut().unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        render_box.render(pixmap);

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
        self = self
            .add_system(update_cursor_position)
            .add_system(handle_mouse_button)
            .add_system(intersection_observer)
            .add_system(handle_key_input)
            .add_system(update_widgets)
            .add_system(handle_widget_event);

        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

// TODO: move these systems into systems module

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;

    #[test]
    fn widget_hover_system() {
        let rect = Rect::new().fixed(100.0, 100.0);

        let state_tracker = StateTracker::new(&rect.build());
        let mut resources = Resources::new();
        resources.insert(state_tracker);
        resources.insert(WindowSize(Size::unit(500.0)));
        resources.insert(rect.build());
        resources.insert(CursorPosition(Position::unit(50.0)));
        resources.insert::<Vec<WidgetEvent>>(Vec::new());

        layout_system(&mut resources);
        intersection_observer(&mut resources);

        let events: &Vec<WidgetEvent> = resources.get().unwrap();
        assert!(events.contains(&WidgetEvent::Hovered(rect.id())));
    }
}
