#![doc = include_str!("../../README.md")]
pub mod colors;
pub mod error;
mod view;
pub mod widgets;

pub use crystal;
use crystal::LayoutSolver;
pub use error::{Error, Result};
pub use helium_core::*;
pub use helium_macros::hex; // TODO move to colors mod
pub use helium_renderer as renderer;
use helium_renderer::{Renderer, TextSurface};
pub use nanoid::nanoid;
use pixels::{Pixels, SurfaceTexture};
use resvg::tiny_skia;
use resvg::tiny_skia::Pixmap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use widgets::Widget;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::event_loop::EventLoopBuilder;
use winit::platform::windows::EventLoopBuilderExtWindows;

/// An [`App`]'s is the point of entry for your program they are responsible
/// for the overall management of rendering, resources,
/// [`Widget`]'s etc.
pub struct App {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
    widget: Box<dyn Widget>,
}

impl App {
    pub fn new(widget: impl Widget + 'static) -> Self {
        // FIXME handle the errors
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop,
            window: Arc::new(window),
            widget: Box::new(widget),
        }
    }
    
    fn layout(&self){
        let layout = self.widget.layout();
    }

    // FIXME app panics if there are no views
    pub async fn run(mut self) -> Result<()> {
        self.window.set_visible(true);

        let mut renderer = Renderer::new(&self.window).await;
        log::info!("Running app");

        // Not quite sure how accurate this is
        let mut previous_duration = Duration::new(0, 0);
        let mut size = Size::from(self.window.inner_size());

        let surface = SurfaceTexture::new(
            size.width as u32,
            size.height as u32,
            Arc::clone(&self.window),
        );
        let mut pixels = Pixels::new(size.width as u32, size.height as u32, surface).unwrap();
        let mut pixmap = Pixmap::new(size.width as u32, size.height as u32).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        self.event_loop.run(|event, window_target| {
            let instant = Instant::now();
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => {
                        // self.widget.render(&mut pixmap);
                        let view = self.widget.view();
                        view.render(&mut pixmap);
                        pixels.frame_mut().copy_from_slice(pixmap.data());
                        pixels.render().unwrap();
                    }
                    WindowEvent::Resized(window_size) => {
                        size = Size::from(window_size);
                        renderer.resize(window_size.into());
                        // I think resizing already causes a redraw request but i'm not sure
                        self.window.request_redraw();
                    }
                    event => {
                        // self.pages[self.index].dispatch_event(&event);
                        self.window.request_redraw();
                    }
                },
                _ => {
                    // self.pages[self.index].update();
                    self.window.request_redraw();
                }
            }
            previous_duration = instant.elapsed();
        })?;

        Ok(())
    }
}

pub struct Page {
    mouse_pos: Position,
    widget: Box<dyn Widget>,
}

impl Page {
    /// Create a new [`Page`]
    pub fn new(widget: impl Widget + 'static, renderer: &mut Renderer) -> Self {
        // let layout = body.layout();
        Self {
            mouse_pos: Position::default(),
            // layout,
            widget: Box::new(widget),
        }
    }

    fn update(&mut self) {
        self.widget.update();
    }


    /// Draw the contents of the [`Page`] to the screen
    fn draw(&mut self, renderer: &mut Renderer, size: Size) {
        // let widget_body = self.widget.build(renderer);

        // Solve the Layout tree
        // let mut layout = widget_body.layout();
        // LayoutSolver::solve(&mut *layout, size);
        // self.layout = layout;

        // let mut primitives = vec![widget_body.primitive()];
        // primitives.extend(widget_body.children().iter().map(|child| child.primitive()));

        // renderer.draw(primitives);

        // self.widget.iter().for_each(|w| {
        // 	if let Some(layout) = layout.get(w.id()) {
        // 		w.draw(layout, renderer);
        //     } else {
        // 		log::warn!("Widget is missing it's Layout")
        //     }
        //     pixels.frame_mut().copy_from_slice(pixmap.data());
        //
        // });
    }
}

#[cfg(test)]
fn test_setup(){
    use winit::platform::windows::WindowBuilderExtWindows;
    
    let event_loop = EventLoopBuilder::new()
        .with_any_thread(true)
        .build()
        .unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let surface = SurfaceTexture::new(1000,1000,window);
    let pixels = Pixels::new(1000,1000,surface).unwrap();
    let pixmap = Pixmap::new(500,500).unwrap();
}
