mod builders;
mod error;
mod pipeline;
mod primitives;
mod vertex;
pub use error::{Error,Result};
pub use helium_core::{IntoColor, Rgba, Size,Color};
use pipeline::{
    CirclePipeline, GlobalResources, IconPipeline, ImagePipeline, RectPipeline, TextPipeline,
};
pub use primitives::*;
use std::{
    rc::Rc,
    time::Instant,
};
pub use vertex::Vertex;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// An `App` is a convience structure that manages
/// the window and event loop, allowing you to simply
/// use the renderer directly.
/// 
/// # Example
/// ```no_run
/// use ruby::{App,Result,RectSurface};
/// 
/// #[tokio::main]
/// async fn main() -> ruby::Result<()>{
/// 	let app = App::new()?;
/// 	app.run(|r|{
/// 		r.draw([RectSurface::new(300.0,300.0)]);
/// 	}).await?;
/// 
/// 	Ok(())
/// }
/// ```
pub struct App {
    event_loop: EventLoop<()>,
    window: Window,
}

impl App {
    pub fn new() -> Result<Self> {
        // FIXME handle the errors
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

		// Set the window to invisible at startup to prevent flashes
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            window,
        })
    }

    pub async fn run(self, mut f: impl FnMut(&mut Renderer)) -> Result<()> {
        self.window.set_visible(true);

        let mut renderer = Renderer::new(&self.window).await;
        log::info!("Running app");

        let mut size = Size::from(self.window.inner_size());

        self.event_loop.run(|event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => {
                        f(&mut renderer);
                        renderer.render();
                    }
                    WindowEvent::Resized(window_size) => {
                        size = Size::from(window_size);
                        renderer.resize(window_size.into());
                        // I think resizing already causes a redraw request but i'm not sure
                        self.window.request_redraw();
                    }
                    _ => {
                        self.window.request_redraw();
                    }
                },
                _ => {
                    self.window.request_redraw();
                }
            }
        })?;

        Ok(())
    }
}

pub struct Renderer<'r> {
    surface: wgpu::Surface<'r>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    global: Rc<GlobalResources>,
    rect_pipeline: RectPipeline,
    circle_pipeline: CirclePipeline,
    text_pipeline: TextPipeline,
    image_pipeline: ImagePipeline,
    icon_pipeline: IconPipeline,
    draw_queue: Vec<Surface>,
}

impl<'r> Renderer<'r> {
    pub async fn new(window: &'r Window) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap(); // FIXME return these errors

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device/Queue"),
                    required_features: wgpu::Features::empty(),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|s| s.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

		// TODO expose this to the user?
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

		// TODO I think wgpu updated so that you can 
		// just clone the resources directly.
        let global = Rc::new(GlobalResources::new(
            &device,
            Size::from(window.inner_size()),
        ));

        let rect_pipeline = RectPipeline::new(&device, config.format, Rc::clone(&global));
        let circle_pipeline = CirclePipeline::new(&device, config.format, Rc::clone(&global));
        let text_pipeline = TextPipeline::new(&device, config.format, Rc::clone(&global));
        let image_pipeline = ImagePipeline::new(&device, config.format, Rc::clone(&global));
        let icon_pipeline = IconPipeline::new(&device, config.format, Rc::clone(&global));

        Self {
            surface,
            device,
            queue,
            config,
            rect_pipeline,
            circle_pipeline,
            text_pipeline,
            image_pipeline,
            icon_pipeline,
            global,
            draw_queue: vec![],
        }
    }

    pub fn text_size(&mut self, text: &TextSurface) -> Size {
        self.text_pipeline.text_size(text)
    }

    pub fn resize(&mut self, size: Size) {
        self.config.width = size.width as u32;
        self.config.height = size.height as u32;

        // Resize the surface with the window to keep the right scale
        self.queue.write_buffer(
            self.global.window_buffer(),
            0,
            bytemuck::cast_slice(&[size]),
        );
        self.surface.configure(&self.device, &self.config);
    }

	pub fn draw_square(&mut self, size: f32,color: impl IntoColor<Rgba>,radius: f32){
		let rect = RectSurface::new(size, size)
			.color(color)
			.corner_radius(radius);

		self.draw_queue.push(primitives::Surface::Rect(rect));
	}

	pub fn draw_image(&mut self, image: primitives::Image){
		self.image_pipeline.draw(image);
	}

	pub fn draw_icon(&mut self, icon: primitives::Icon){
		self.icon_pipeline.draw(icon);
	}

	pub fn draw_circle(&mut self, circle: primitives::Circle){
		self.circle_pipeline.draw(circle);
	}

    /// Add primitives to the draw queue
    pub fn draw<I, P>(&mut self, primitives: I)
    where
        I: IntoIterator<Item = P>,
        P: IntoSurface,
    {
        self.draw_queue
            .extend(primitives.into_iter().map(|p| p.into_surface()));
    }

    pub fn render(&mut self) {
        let instant = Instant::now();
        let output = self.surface.get_current_texture().unwrap(); // TODO maybe handle this error
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        for primitive in self.draw_queue.drain(..) {
            match primitive {
                Surface::Rect(rect) => {
                    self.rect_pipeline
                        .draw(&rect, &self.device, &mut render_pass);
                }
                Surface::Text(text) => {
                    self.text_pipeline
                        .draw(&text, &self.queue, &self.device, &mut render_pass);
                }
            }
        }

		self.icon_pipeline.render(&self.queue, &self.device, &mut render_pass);
		self.image_pipeline.render(&self.queue, &self.device, &mut render_pass);
		self.circle_pipeline.render(&self.device, &mut render_pass);


        // Drop the render pass because it borrows encoder mutably
        std::mem::drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

#[cfg(test)]
/// Set up the `wgpu::Device` and `wgpu::Queue` for testing
pub(crate) async fn setup() -> (wgpu::Device, wgpu::Queue) {
    use winit::platform::windows::EventLoopBuilderExtWindows;
    use winit::window::WindowBuilder;

    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .with_any_thread(true)
        .build()
        .expect("Failed to create EventLoop");

    let window = WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop)
        .expect("Failed to create window");

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: Default::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device/Queue"),
                required_features: wgpu::Features::empty(),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    (device, queue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn setup_works() {
        let (_device, _queue) = setup().await;
    }
}
