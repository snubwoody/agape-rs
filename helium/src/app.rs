use crate::page::Page;
use crate::Size;
use helium_renderer::Renderer;
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// [`App`]'s contain the whole program and are the point of entry for helium
/// they are responsible for the overall management of rendering, resources,
/// [`Widget`]'s etc.
pub struct App{
    event_loop: EventLoop<()>,
    window: Window,
    pages: Vec<Page>,
    index: usize,
}

impl App {
    pub fn new() -> Self {
        // FIXME handle the errors
        let event_loop = EventLoop::new().unwrap();

        // Set the event loop to always start a new
        // iteration even if there are no events.
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
		

        Self {
            event_loop,
            window,
            pages: vec![],
            index: 0,
        }
    }

    pub fn add_page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    // FIXME app panics if there are no views
    pub fn run(mut self) -> Result<(), crate::Error> {
        self.window.set_visible(true);

		let mut renderer = async_std::task::block_on(Renderer::new(&self.window));

        self.event_loop
            .run(|event, window_target| match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => {
						self.pages[0].draw(&mut renderer);
						renderer.render();
					}
                    WindowEvent::Resized(size) => {
                        self.pages[self.index].resize(Size::from(size));
						renderer.resize(size.into());

						// I think resizing already causes a redraw request but i'm not sure
                        self.window.request_redraw(); 
                    }
                    event => self.pages[self.index].handle(&event),
                },
                _ => {}
            })
            .expect("Event loop error occured");
        // TODO return this error

        Ok(())
    }
}
