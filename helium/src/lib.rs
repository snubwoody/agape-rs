//! GUI library
pub mod colors;
pub mod error;
mod view;
pub mod widgets;

use crate::view::View;
pub use crystal;
use crystal::LayoutSolver;
pub use error::{Error, Result};
pub use helium_core::*;
pub use helium_macros::hex; // TODO move to colors mod
pub use nanoid::nanoid;
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

/// An [`App`]'s is the point of entry for your program they are responsible
/// for the overall management of rendering, resources,
/// [`Widget`]'s etc.
pub struct App<'app> {
    widget: Box<dyn Widget>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    pixmap: Option<Pixmap>,
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
        match event {
            WindowEvent::CloseRequested => {
                println!("Exiting app");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
                self.window.as_mut().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

impl App<'_> {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            widget: Box::new(widget),
            window: None,
            pixmap: None,
            pixels: None,
        }
    }
    
    fn render(&mut self) {
        let pixmap = self.pixmap.as_mut().unwrap();
        let pixels = self.pixels.as_mut().unwrap();
        let mut view = self.widget.view();
        let views: Vec<Box<dyn View>> = self.widget
            .iter()
            .map(|w|w.view())
            .collect();

        let mut layout = self.widget.layout();
        LayoutSolver::solve(
            &mut *layout,
            self.window.as_ref().unwrap().inner_size().into(),
        );
        let layout = layout.get(view.id()).unwrap();
        view.set_size(layout.size());
        view.set_position(layout.position());

        view.render(pixmap);
        pixels.frame_mut().copy_from_slice(pixmap.data());
        pixels.render().unwrap();
    }

    pub fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}
