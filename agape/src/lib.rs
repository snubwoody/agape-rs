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
use crate::resources::{CursorPosition, EventQueue, WindowSize};

pub type StateMap = HashMap<GlobalId,WidgetState>;

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
        // FIXME update surface on resizing
        log::trace!("WindowEvent: {event:?}");
        self.event_queue.push(event.clone());

        for system in self.systems.iter_mut() {
            system.run(&mut self.resources,&self.event_queue);
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
            }
            WindowEvent::MouseInput { .. } => {}
            _ => {}
        }

        self.event_queue.clear();
    }
}

impl App<'_> {
    pub fn new(widget: impl Widget + 'static) -> Self {
        let len = widget.iter().count();
        log::info!("Creating widget tree with {len} widgets");
        
        let layout = widget.layout();
        let widget: Box<dyn Widget> = Box::new(widget);
        let mut state_map: StateMap = HashMap::new();
        state_map.insert(widget.id(),WidgetState::Resting);
        
        let mut resources = Resources::new();
        resources.insert(CursorPosition::default());
        resources.insert(WindowSize::default());
        resources.insert(layout);
        resources.insert(EventQueue::new());
        resources.insert(widget);
        resources.insert(state_map);
        
        let systems = vec![
            Box::new(layout_system.into_system()) as Box<dyn System>,
        ];

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
        let widget = self.resources.get::<Box<dyn Widget>>().unwrap();
        let mut views: Vec<Box<dyn View>> = widget.iter().map(|w| w.view()).collect();
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
        self = 
            self.add_system(update_cursor_position)
                .add_system(intersection_observer);
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

fn layout_system(resources: &mut Resources) {
    let WindowSize(size) = resources.get_owned::<WindowSize>().unwrap();
    
    let layout: &mut Box<dyn Layout> = resources.get_mut().unwrap();
    LayoutSolver::solve(&mut **layout, size);
}

fn update_cursor_position(resources: &mut Resources,event: &WindowEvent) {
    if let WindowEvent::CursorMoved{ position,..} = event {
        let cursor_position = resources.get_mut::<CursorPosition>().unwrap();    
        cursor_position.0 = Position::from(*position);
    }
}

fn intersection_observer(resources: &mut Resources) {
    let cursor_pos = resources.get::<CursorPosition>().unwrap();
    let layout = resources.get::<Box<dyn Layout>>().unwrap();
    let hovered_ids: Vec<GlobalId> = layout.iter()
        .filter(|l|l.bounds().within(&cursor_pos.0))
        .map(|l|l.id())
        .collect();
    
    let state_map: &mut StateMap = resources.get_mut().unwrap();
    for id in hovered_ids{
        state_map.insert(id,WidgetState::Hovered);
        dbg!("Hovered");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hstack;
    use crate::widgets::Rect;

    #[test]
    fn widget_hover_system(){
        let rect = Rect::new(100.0,100.0);
        let mut layout = rect.layout();
        LayoutSolver::solve(&mut *layout,Size::unit(500.0));
        
        let mut state_map: HashMap<GlobalId,WidgetState> = HashMap::new();
        state_map.insert(rect.id(),WidgetState::Resting);
        
        let mut resources = Resources::new();
        resources.insert(layout);
        resources.insert(state_map);
        resources.insert(CursorPosition(Position::unit(50.0)));
        
        intersection_observer(&mut resources);
        
        let state_map: &StateMap = resources.get().unwrap();
        assert_eq!(state_map.get(&rect.id()).unwrap(), &WidgetState::Hovered);
    }
    
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
        app.resources.get::<EventQueue>().unwrap();
        app.resources.get::<Box<dyn Widget>>().unwrap();
    }
    
    #[test]
    fn init_systems() {
        let app = App::new(hstack! {});
        assert_eq!(app.systems.len(), 1);
    }
}
