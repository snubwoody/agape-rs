//! A GUI library.
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
pub mod view;
pub mod widgets;

use crate::resources::{CursorPosition, EventQueue, WindowSize};
use crate::view::{View, init_font};
use crate::widgets::{RenderBox, StateTracker, WidgetEvent, WidgetState};
pub use agape_core::*;
pub use agape_layout as layout;
use agape_layout::{Layout, LayoutSolver};
pub use agape_macros::hex;
pub use error::{Error, Result};
pub use resources::Resources;
use system::{IntoSystem, System};
use widgets::Widget;

use fontdue::Font;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use std::sync::OnceLock;
use tiny_skia::Pixmap;
use winit::event_loop::ActiveEventLoop;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
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

        let state_tracker = StateTracker::new(&widget);
        let widget: Box<dyn Widget> = Box::new(widget);
        let render_box = widget.build();
        let layout = render_box.layout();

        let mut resources = Resources::new();
        resources.insert(state_tracker);
        resources.insert(render_box);
        resources.insert(CursorPosition::default());
        resources.insert(WindowSize::default());
        resources.insert(layout);
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
        let mut pixmap = self.pixmap.as_mut().unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        render_box.render(&mut pixmap, &self.resources);

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
            .add_system(handle_widget_event);

        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

fn layout_system(resources: &mut Resources) {
    let WindowSize(size) = resources.get_owned::<WindowSize>().unwrap();

    let render_box = resources.get_mut::<RenderBox>().unwrap();
    render_box.solve_layout(size);
}

fn update_cursor_position(resources: &mut Resources, event: &WindowEvent) {
    if let WindowEvent::CursorMoved { position, .. } = event {
        let cursor_position = resources.get_mut::<CursorPosition>().unwrap();
        cursor_position.0 = Position::from(*position);
    }
}

fn handle_mouse_button(resources: &mut Resources, event: &WindowEvent) {
    match event {
        &WindowEvent::MouseInput { state, button, .. } => {
            if state != ElementState::Pressed || button != MouseButton::Left {
                return;
            }
        }
        _ => return,
    }

    let layout = resources.get::<Box<dyn Layout>>().unwrap();
    let cursor_position: &CursorPosition = resources.get().unwrap();

    let ids: Vec<GlobalId> = layout
        .iter()
        .filter(|l| l.bounds().within(&cursor_position.0))
        .map(|l| l.id())
        .collect();

    let state_tracker = resources.get_mut::<StateTracker>().unwrap();
    for id in &ids {
        state_tracker.update_state(*id, WidgetState::Clicked);
    }

    let event_queue = resources.get_mut::<Vec<WidgetEvent>>().unwrap();
    for id in ids {
        event_queue.push(WidgetEvent::Clicked(id));
    }
}

fn handle_key_input(resources: &mut Resources, event: &WindowEvent) {
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

fn intersection_observer(resources: &mut Resources) {
    let cursor_pos = resources.get::<CursorPosition>().unwrap();
    let layout = resources.get::<Box<dyn Layout>>().unwrap();

    // TODO combine both iters and just use a for loop
    let hovered_ids: Vec<GlobalId> = layout
        .iter()
        .filter(|l| l.bounds().within(&cursor_pos.0))
        .map(|l| l.id())
        .collect();

    let not_hovered: Vec<GlobalId> = layout
        .iter()
        .filter(|l| !hovered_ids.contains(&l.id()))
        .map(|l| l.id())
        .collect();

    let state = resources.get_mut::<StateTracker>().unwrap();
    for id in &hovered_ids {
        state.update_state(*id, WidgetState::Hovered);
    }

    for id in &not_hovered {
        state.update_state(*id, WidgetState::Resting);
    }

    let state = resources.get::<StateTracker>().unwrap();
    let mut events = vec![];
    for id in &hovered_ids {
        if state.previous_state(*id).unwrap() == &WidgetState::Resting {
            events.push(WidgetEvent::Hovered(*id));
        }
    }

    let widget_events: &mut Vec<WidgetEvent> = resources.get_mut().unwrap();
    widget_events.extend(events);
}

fn handle_widget_event(resources: &mut Resources) {
    let events: Vec<WidgetEvent> = resources.get_owned().unwrap();
    let widget: &mut Box<dyn Widget> = resources.get_mut().unwrap();

    for event in events {
        widget.handle_event(&event);
    }

    resources.get_mut::<Vec<WidgetEvent>>().unwrap().clear();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hstack;
    use crate::widgets::Rect;

    #[test]
    fn reconstruct_layout_every_frame() {
        let hstack = hstack! {}.fill();
        let widget: Box<dyn Widget> = Box::new(hstack);

        let mut resources = Resources::new();
        resources.insert(widget);
        resources.insert(WindowSize(Size::unit(500.0)));

        layout_system(&mut resources);

        let layout = resources.get::<Box<dyn Layout>>().unwrap();
        assert_eq!(layout.size(), Size::unit(500.0));
    }

    #[test]
    fn widget_hover_system() {
        let rect = Rect::new().fixed(100.0, 100.0);

        let state_tracker = StateTracker::new(&rect);
        let mut resources = Resources::new();
        resources.insert(state_tracker);
        resources.insert(CursorPosition(Position::unit(50.0)));
        resources.insert::<Vec<WidgetEvent>>(Vec::new());

        intersection_observer(&mut resources);

        let events: &Vec<WidgetEvent> = resources.get().unwrap();
        assert!(events.contains(&WidgetEvent::Hovered(rect.id())));
    }

    #[test]
    fn layout_system_works() {
        let hstack = hstack! {}.fill();
        let widget: Box<dyn Widget> = Box::new(hstack);

        let mut resources = Resources::new();
        resources.insert(widget);
        resources.insert(WindowSize(Size::unit(500.0)));

        layout_system(&mut resources);

        let layout = resources.get::<Box<dyn Layout>>().unwrap();
        assert_eq!(layout.size(), Size::unit(500.0));
    }

    #[test]
    fn initial_resources() {
        let app = App::new(hstack! {});
        app.resources.get::<CursorPosition>().unwrap();
        app.resources.get::<WindowSize>().unwrap();
        app.resources.get::<Box<dyn Layout>>().unwrap();
        app.resources.get::<EventQueue>().unwrap();
        app.resources.get::<Box<dyn Widget>>().unwrap();
        app.resources.get::<Vec<WidgetEvent>>().unwrap();
        app.resources.get::<StateTracker>().unwrap();

        assert_eq!(app.resources.len(), 7);
    }

    #[test]
    fn init_systems() {
        let app = App::new(hstack! {});
        assert_eq!(app.systems.len(), 1);
    }
}
