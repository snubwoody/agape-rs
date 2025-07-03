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
mod context;
pub mod error;
mod macros;
pub mod view;
pub mod widgets;
mod system;

use crate::view::View;
pub use context::Context;
pub use crystal;
use crystal::LayoutSolver;
pub use error::{Error, Result};
pub use helium_core::*;
pub use helium_macros::hex;
use pixels::{Pixels, SurfaceTexture};
use resvg::tiny_skia::Pixmap;
use std::sync::Arc;
use std::time::Instant;
use widgets::Widget;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

type System = Box<dyn FnMut(&mut Context)>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppEvent {
    /// Emitted when the cursor is over a widget
    Hovered(GlobalId),
    /// Emitted when the left mouse button is pressed
    /// while the [`Widget`] is in a hovered state.
    Clicked(GlobalId)
}

pub struct App<'app> {
    widget: Box<dyn Widget>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    pixmap: Option<Pixmap>,
    context: Context,
    systems: Vec<System>,
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
        
        self.context.window_size = size;
        self.pixels = Some(pixels);
        self.window = Some(Arc::clone(&window));
        self.pixmap = Some(pixmap);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // FIXME update surface on resizing
        log::trace!("WindowEvent: {:?}", event);

        self.context.handle_event(event.clone());
        
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
                self.pixels.as_mut()
                    .unwrap()
                    .resize_surface(size.width, size.height)
                    .expect("Failed to resize the pixel buffer");
            }
            WindowEvent::MouseInput { .. } => {}
            _ => {}
        }

        for system in self.systems.iter_mut() {
            system(&mut self.context)
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

        let views = widget.iter().map(|w|w.view()).collect();
        let systems: Vec<System> = vec![
            Box::new(layout_system)
        ];
        
        Self {
            context: Context::new(&widget,views),
            widget: Box::new(widget),
            window: None,
            pixmap: None,
            pixels: None,
            systems
        }
    }
    
    pub fn add_system(mut self, system: impl FnMut(&mut Context) + 'static) -> Self{
        self.systems.push(Box::new(system));
        self
    }

    fn render(&mut self) {
        let mut views: Vec<Box<dyn View>> = self.widget.iter().map(|w| w.view()).collect();

        let mut layout = self.context.layout();
        // LayoutSolver::solve(
        //     &mut *layout,
        //     self.window.as_ref().unwrap().inner_size().into(),
        // );

        let pixels = self.pixels.as_mut().unwrap();
        let pixmap = self.pixmap.as_mut().unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        // Draw each view(widget) to the pixmap
        for view in &mut views {
            // TODO don't unwrap
            let layout = layout.get(view.id()).unwrap();
            view.set_size(layout.size());
            view.set_position(layout.position());
            view.render(pixmap);
            pixels.frame_mut().copy_from_slice(pixmap.data());
        }

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

fn layout_system(cx: &mut Context) {
    let instant = Instant::now();
    let size = cx.window_size;
    LayoutSolver::solve(cx.layout_mut(), size);
    println!("Frame time: {:?}",instant.elapsed());
}
