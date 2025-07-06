//! A GUI library that feels like writing regular rust.
//!
//! # Getting started
//! ```no_run
//! use agape::{App,hstack,widgets::Text};
//!
//! let hstack = hstack! {
//!     Text::new("Hello"),
//!     Text::new("world")
//! }
//! .padding(12)
//! .spacing(12)
//! .align_center();
//!
//! let app = App::new(hstack);
//! app.run().unwrap();
//! ```
//!
//! # Architecture
//! Widgets are high level objects that can contain any kind of data like text, buttons and
//! scrollbars.
//!
//! Layouts describe how the widgets prefer to be arranged, like the size, position, vertical or
//! horizontal, and so on.
//!
//! Views are the final item and hold only basic information, like color, size and position, and
//! are responsible for drawing the widgets to the screen.
//!
pub mod error;
mod macros;
pub mod system;
pub mod view;
pub mod widgets;
pub mod resources;

pub use resources::Resources;
use crate::view::View;
use crate::widgets::WidgetState;
pub use agape_core::*;
pub use agape_layout;
use agape_layout::{Layout, LayoutSolver};
pub use agape_macros::hex;
pub use error::{Error, Result};
use pixels::{Pixels, SurfaceTexture};
use tiny_skia::Pixmap;
use std::collections::HashMap;
use std::sync::Arc;
use system::{IntoSystem, System};
use widgets::Widget;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use crate::resources::{CursorPosition, WindowSize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppEvent {
    /// Emitted when the cursor is over a widget
    Hovered(GlobalId),
    /// Emitted when the left mouse button is pressed
    /// while the [`Widget`] is in a hovered state.
    Clicked(GlobalId),
}

/// An `App` is a single program.
pub struct App<'app> {
    widget: Box<dyn Widget>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    pixmap: Option<Pixmap>,
    context: Context,
    resources: Resources,
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

        self.context.window_size = size;
        self.pixels = Some(pixels);
        self.window = Some(Arc::clone(&window));
        self.pixmap = Some(pixmap);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // FIXME update surface on resizing
        log::trace!("WindowEvent: {event:?}");

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
            }
            WindowEvent::MouseInput { .. } => {}
            _ => {}
        }

        for system in self.systems.iter_mut() {
            system.run(&mut self.resources)
        }

        self.widget.tick(&self.context);
        self.context.clear_events();
    }
}

impl App<'_> {
    pub fn new(widget: impl Widget + 'static) -> Self {
        let len = widget.iter().count();
        log::info!("Creating widget tree with {len} widgets");
        
        let layout = widget.layout();
        let mut resources = Resources::new();
        resources.insert(CursorPosition::default());
        resources.insert(WindowSize::default());
        resources.insert(layout);
        
        let systems = vec![Box::new(layout_system.into_system()) as Box<dyn System>];

        Self {
            context: Context::new(&widget),
            widget: Box::new(widget),
            window: None,
            pixmap: None,
            resources,
            pixels: None,
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
    pub fn add_system(mut self, f: impl IntoSystem + 'static) -> Self {
        self.systems.push(Box::new(f.into_system()));
        self
    }

    fn render(&mut self) {
        let mut views: Vec<Box<dyn View>> = self.widget.iter().map(|w| w.view()).collect();
        let layout = self.resources.get::<Box<dyn Layout>>().unwrap();
        // let mut views = &mut self.context.views;

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
            // TODO copy data after
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

/// Global app context which keeps track of important
/// information such as the current mouse position and
/// the state of each widget.
pub struct Context {
    mouse_position: Position,
    /// Keeps track of the state of all widgets in the
    /// widget tree.
    state: HashMap<GlobalId, WidgetState>,
    layout: Box<dyn Layout>,
    events: Vec<AppEvent>,
    pub window_size: Size,
}

impl Context {
    /// Create a new context object
    pub fn new(widget: &impl Widget) -> Self {
        let mut state = HashMap::new();
        for w in widget.iter() {
            state.insert(w.id(), WidgetState::Resting);
        }
        let layout = widget.layout();

        Self {
            mouse_position: Position::default(),
            layout,
            state,
            window_size: Size::default(),
            events: Vec::new(),
        }
    }

    pub fn query_events(&self) -> &[AppEvent] {
        self.events.as_slice()
    }

    pub(crate) fn clear_events(&mut self) {
        self.events.clear();
    }

    pub fn layout(&self) -> &dyn Layout {
        &*self.layout
    }

    pub fn layout_mut(&mut self) -> &mut dyn Layout {
        &mut *self.layout
    }

    pub fn mouse_pos(&self) -> Position {
        self.mouse_position
    }

    pub fn state(&self) -> &HashMap<GlobalId, WidgetState> {
        &self.state
    }

    /// Get the state of a [`Widget`].
    pub fn get_state(&self, id: &GlobalId) -> Option<&WidgetState> {
        self.state.get(id)
    }

    /// Set the state of a [`Widget`].
    pub fn set_state(&mut self, id: GlobalId, state: WidgetState) {
        self.state.insert(id, state);
    }
}

fn layout_system(resources: &mut Resources) {
    let WindowSize(size) = resources.get_owned::<WindowSize>().unwrap();
    
    let layout: &mut Box<dyn Layout> = resources.get_mut().unwrap();
    LayoutSolver::solve(&mut **layout, size);
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::hstack;
    
    #[test]
    fn layout_system_works(){
        let hstack = hstack!{}.fill();
        let layout = hstack.layout();
        
        let mut resources = Resources::new();
        resources.insert(layout);
        resources.insert(WindowSize(Size::unit(500.0)));
    
        layout_system(&mut resources);
        
        let layout  = resources.get::<Box<dyn Layout>>().unwrap();
        assert_eq!(layout.size(),Size::unit(500.0));
    }

    #[test]
    fn initial_resources() {
        let app = App::new(hstack! {});
        app.resources.get::<CursorPosition>().unwrap();
        app.resources.get::<WindowSize>().unwrap();
        app.resources.get::<Box<dyn Layout>>().unwrap();
    }
    
    #[test]
    fn init_systems() {
        let app = App::new(hstack! {});
        assert_eq!(app.systems.len(), 1);
    }
}
