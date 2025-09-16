//! A cross-platform GUI library.
//!
//! ## Getting started
//! To get started you'll need to create an [`App`], which is the entry point
//! of the program, and a root [`View`].
//!
//! ```no_run
//! use agape::{App,Error,widgets::*};
//!
//! fn main() -> Result<(),Error>{
//!     App::new(Main)
//!         .run()
//! }
//!
//! struct Main;
//!
//! impl View for Main{
//!     fn view(&self) -> Box<dyn Widget> {
//!         Box::new(Text::new("Hello!"))
//!     }
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

use crate::widgets::{ViewTree, WidgetTree, update_hovered_state};
use crate::widgets::{click_widget, get_assets};
pub use agape_core::*;
pub use agape_layout as layout;
pub use agape_macros::hex;
pub use agape_renderer as renderer;
pub use error::{Error, Result};
pub use message::MessageQueue;
use message::update_cursor_pos;
use resources::CursorPosition;
use resources::EventQueue;
use std::path::Path;
use widgets::View;

use crate::assets::AssetManager;
use crate::message::{MouseButtonDown, MouseButtonUp};
use crate::state::State;
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

#[derive(Resource)]
struct LayoutTree(Box<dyn Layout>);

#[derive(Resource)]
struct WindowSize(Size);

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
    pub fn new(view: impl View + 'static) -> Self {
        let state = State::new(view);

        Self {
            window: None,
            pixels: None,
            state,
        }
    }

    pub fn assets(mut self, path: impl AsRef<Path>) -> Self {
        // FIXME
        self.state.asset_dir(path);
        // self.world.insert_resource(AssetManager::new(path));
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
        // self.schedule
        //     .add_systems(update_window_size)
        //     .add_systems(update_cursor_pos)
        //     .add_systems(handle_click)
        //     .add_systems((get_assets, update_layout, render_widget, update_view).chain())
        //     // .add_systems(get_assets)
        //     .add_systems(update_hovered_state)
        //     .add_systems(click_widget)
        //     .add_systems(clear_events);

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

        self.pixels = Some(pixels);
        self.window = Some(Arc::clone(&window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // self.world
        //     .get_resource_mut::<EventQueue>()
        //     .unwrap()
        //     .push(event.clone());
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Exiting app");
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
            _ => {}
        }
        self.state.update();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::*;

    struct DummyView;
    impl View for DummyView {
        fn view(&self) -> Box<dyn Widget> {
            Box::new(Text::new(""))
        }
    }

    #[test]
    fn important_resources() {
        panic!("No longer valid");
        // Make sure some of the important resources are present
        // let app = App::new(DummyView);
        // app.world.get_resource::<WindowSize>().unwrap();
        // app.world.get_resource::<CursorPosition>().unwrap();
        // app.world.get_resource::<ViewTree>().unwrap();
        // app.world.get_resource::<WidgetTree>().unwrap();
        // app.world.get_resource::<LayoutTree>().unwrap();
        // app.world.get_resource::<EventQueue>().unwrap();
        // app.world.get_resource::<MessageQueue>().unwrap();
    }
}
