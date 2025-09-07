//! A cross-platform GUI library.
//!
//! ## Getting started
//! To get started you'll need to create an [`App`], which is the entry point
//! of the program, and a root [`Widget`].
pub mod error;
mod macros;
pub mod message;
pub mod resources;
pub mod style;
pub mod widgets;

use crate::widgets::spawn_widget_gestures;
use crate::widgets::{ViewTree, WidgetTree, update_hovered_state};
pub use agape_core::*;
pub use agape_layout as layout;
pub use agape_macros::hex;
pub use agape_renderer as renderer;
pub use error::{Error, Result};
pub use message::MessageQueue;
use message::update_cursor_pos;
use resources::CursorPosition;
use resources::EventQueue;
use widgets::View;

use crate::message::{MouseButtonDown, MouseButtonUp};
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

// TODO: store the pixmap in the renderer?
/// An `App` is a single program.
pub struct App<'app> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'app>>,
    world: World,
    schedule: Schedule,
}

impl App<'_> {
    /// Create a new app.
    pub fn new(view: impl View + 'static) -> Self {
        let widget = view.view();
        let mut renderer = Renderer::new();
        let layout = widget.layout(&mut renderer);
        let view_tree = ViewTree(Box::new(view));

        let mut world = World::new();
        world.insert_resource(CursorPosition::default());
        world.insert_resource(WidgetTree(widget));
        world.insert_resource(CursorPosition::default());
        world.insert_resource(EventQueue::default());
        world.insert_resource(MessageQueue::default());
        world.insert_resource(view_tree);
        world.insert_resource(renderer);
        world.insert_resource(LayoutTree(layout));
        world.insert_resource(WindowSize(Size::unit(1.0)));

        Self {
            window: None,
            pixels: None,
            world,
            schedule: Schedule::default(),
        }
    }

    fn render(&mut self) {
        let renderer = self.world.get_resource::<Renderer>().unwrap();
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
        self.schedule
            .add_systems(update_cursor_pos)
            .add_systems(handle_click)
            .add_systems(update_window_size)
            .add_systems(render_widget)
            .add_systems(update_layout)
            .add_systems(update_view)
            .add_systems(spawn_widget_gestures)
            .add_systems(update_hovered_state)
            .add_systems(clear_events);

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
        self.world
            .get_resource_mut::<EventQueue>()
            .unwrap()
            .push(event.clone());
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

                self.world
                    .get_resource_mut::<Renderer>()
                    .unwrap()
                    .resize(size.width, size.height);
            }
            _ => {}
        }
        self.schedule.run(&mut self.world);
    }
}

// TODO: test these
fn update_window_size(event_queue: Res<EventQueue>, mut window_suze: ResMut<WindowSize>) {
    for event in event_queue.events() {
        if let WindowEvent::Resized(size) = event {
            window_suze.0 = Size::from(*size);
        }
    }
}

fn update_view(
    mut view_tree: ResMut<ViewTree>,
    mut messages: ResMut<MessageQueue>,
    mut widget_tree: ResMut<WidgetTree>,
) {
    let view = &mut view_tree.0;
    view.update(&mut messages);
    widget_tree.0 = view.view();
}

fn update_layout(
    widget_tree: Res<WidgetTree>,
    window_size: Res<WindowSize>,
    mut renderer: ResMut<Renderer>,
    mut layout_tree: ResMut<LayoutTree>,
) {
    let widget = &widget_tree.0;
    let mut layout = widget.layout(&mut renderer);
    solve_layout(&mut *layout, window_size.0);
    layout_tree.0 = layout;
}

fn render_widget(
    widget_tree: Res<WidgetTree>,
    mut renderer: ResMut<Renderer>,
    layout_tree: Res<LayoutTree>,
) {
    let widget = &widget_tree.0;

    renderer.pixmap_mut().fill(tiny_skia::Color::WHITE);
    widget.render(&mut renderer, layout_tree.0.as_ref());
}

fn handle_click(queue: ResMut<EventQueue>, mut messages: ResMut<MessageQueue>) {
    for event in queue.events() {
        if let WindowEvent::MouseInput { button, state, .. } = event {
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
    }
}

fn clear_events(mut queue: ResMut<EventQueue>, mut messages: ResMut<MessageQueue>) {
    messages.tick();
    messages.clear();
    queue.clear();
    queue.tick();
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
        // Make sure some of the important resources are present
        let app = App::new(DummyView);
        app.world.get_resource::<WindowSize>().unwrap();
        app.world.get_resource::<CursorPosition>().unwrap();
        app.world.get_resource::<ViewTree>().unwrap();
        app.world.get_resource::<WidgetTree>().unwrap();
        app.world.get_resource::<LayoutTree>().unwrap();
        app.world.get_resource::<EventQueue>().unwrap();
        app.world.get_resource::<MessageQueue>().unwrap();
    }
}
