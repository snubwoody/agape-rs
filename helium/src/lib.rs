//! GUI library
//!
//! ## Architecture
//! Widgets are high level objects that can contain any kind of data like text, buttons and
//! scrollbars.
//!
//! Layouts describe how the widgets prefer to be arranged, like the size, position, vertical or
//! horizontal, and so on.
//!
//! Views are the final item and hold only basic information, like color, size and position, and
//! are responsible for drawing the widgets to the screen.
//!
//! You will most likely only have to deal with widgets unless you are creating your own widgets.
//!
//! ## Creating custom widgets
//!
//! There are two ways of creating custom widgets, functions and structs. Prefer functions if
//! you just need a wrapper around existing widgets, if you need highly custom functionality
//! then you may implement the [`Widget`] trait yourself.
pub mod error;
mod macros;
pub mod view;
pub mod widgets;
mod context;

use crate::view::View;
pub use crystal;
use crystal::LayoutSolver;
pub use error::{Error, Result};
pub use helium_core::*;
pub use helium_macros::hex; 
pub use context::Context;
use pixels::{Pixels, SurfaceTexture};
use resvg::tiny_skia::Pixmap;
use std::sync::Arc;
use widgets::Widget;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum AppEvent{
    /// Emitted when the cursor is over a widget
    Hovered(GlobalId),
}

pub struct App<'app> {
    widget: Box<dyn Widget>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    pixmap: Option<Pixmap>,
    context: Context,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Initializing resources");
        let window = event_loop.create_window(Default::default()).unwrap();
        let window = Arc::new(window);
        let size = Size::from(window.inner_size());

        let surface =
            SurfaceTexture::new(size.width as u32, size.height as u32, Arc::clone(&window));
        let pixels = Pixels::new(size.width as u32, size.height as u32, surface).unwrap();
        let pixmap = Pixmap::new(size.width as u32, size.height as u32).unwrap();
        self.pixels = Some(pixels);
        self.window = Some(Arc::clone(&window));
        self.pixmap = Some(pixmap);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // FIXME update surface on resizing
        log::trace!("WindowEvent: {:?}", event);
        
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Exiting app");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.as_mut().unwrap().request_redraw();
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.context.update_mouse_pos(position.into());
            },
            WindowEvent::MouseInput { button, state, .. } => {
            }
            _ => {
            }
        }

        self.context.update_state();
        self.widget.tick(&self.context);
        self.context.clear_events();
    }
}

impl App<'_> {
    pub fn new(widget: impl Widget + 'static) -> Self {
        let len = widget.iter().count();
        log::info!("Creating widget tree with {} widgets", len);
        
        Self {
            context: Context::new(&widget),
            widget: Box::new(widget),
            window: None,
            pixmap: None,
            pixels: None,
        }
    }

    fn render(&mut self) {
        let mut views: Vec<Box<dyn View>> = self.widget.iter().map(|w| w.view()).collect();

        let mut layout = self.widget.layout();
        LayoutSolver::solve(
            &mut *layout,
            self.window.as_ref().unwrap().inner_size().into(),
        );

        let pixels = self.pixels.as_mut().unwrap();
        let pixmap = self.pixmap.as_mut().unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);
        
        // Draw each view(widget) to the pixmap
        for view in &mut views {
            let layout = layout.get(view.id()).unwrap();
            view.set_size(layout.size());
            view.set_position(layout.position());
            view.render(pixmap);
            pixels.frame_mut().copy_from_slice(pixmap.data());
        }

        self.context.set_layout(layout);
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




