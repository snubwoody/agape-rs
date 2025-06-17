mod builders;
mod error;
mod pipeline;
mod primitives;
mod vertex;
pub use error::Error;
use helium_core::Size;
use pipeline::{
    CirclePipeline, GlobalResources, IconPipeline, ImagePipeline, RectPipeline, TextPipeline,
};
pub use primitives::*;
use std::{
    rc::Rc,
    time::{Duration, Instant},
};
use pixels::Pixels;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::platform::windows::EventLoopBuilderExtWindows;
pub use vertex::Vertex;
use winit::window::{Window, WindowBuilder};

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

        // TODO maybe return these?
        let mut rect_trace = Duration::new(0, 0);
        let mut circle_trace = Duration::new(0, 0);
        let mut text_trace = Duration::new(0, 0);
        let mut icon_trace = Duration::new(0, 0);
        let mut image_trace = Duration::new(0, 0);

        for primitive in self.draw_queue.drain(..) {
            match primitive {
                Surface::Rect(rect) => {
                    let instant = Instant::now();
                    self.rect_pipeline
                        .draw(&rect, &self.device, &mut render_pass);
                    rect_trace += instant.elapsed();
                }
                Surface::Circle(circle) => {
                    let instant = Instant::now();
                    self.circle_pipeline
                        .draw(&circle, &self.device, &mut render_pass);
                    circle_trace += instant.elapsed();
                }
                Surface::Text(text) => {
                    let instant = Instant::now();
                    self.text_pipeline
                        .draw(&text, &self.queue, &self.device, &mut render_pass);
                    text_trace += instant.elapsed();
                }
                Surface::Image(image) => {
                    let instant = Instant::now();
                    self.image_pipeline
                        .draw(&image, &self.queue, &self.device, &mut render_pass);
                    image_trace += instant.elapsed();
                }
                Surface::Icon(icon) => {
                    let instant = Instant::now();
                    self.icon_pipeline
                        .draw(&icon, &self.queue, &self.device, &mut render_pass);
                    icon_trace += instant.elapsed();
                }
            }
        }

        // Drop the render pass because it borrows encoder mutably
        std::mem::drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        let mut trace = format!("\nFrame time: {:?}\n", instant.elapsed());
        trace += "Trace\n";
        trace += &format!("Icon Pipeline: {icon_trace:?}\n");
        trace += &format!("Image Pipeline: {image_trace:?}\n");
        trace += &format!("Text Pipeline: {text_trace:?}\n");
        trace += &format!("Rect Pipeline: {rect_trace:?}\n");
        trace += &format!("Circle Pipeline: {circle_trace:?}\n");

        //log::trace!("{trace}");
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

pub fn render(){
    dbg!("Building window");
    use winit::platform::windows::EventLoopBuilderExtWindows;
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = WindowBuilder::new()
        .with_visible(true)
        .build(&event_loop)
        .unwrap();
    dbg!("Finished building window");

    let surface = pixels::SurfaceTexture::new(500,500,window);
    let mut pixels = Pixels::new(500,500, surface).unwrap();
    let mut pixmap = Pixmap::new(500,500).unwrap();
    pixmap.fill(tiny_skia::Color::WHITE);

    dbg!("Running app");
    event_loop.run(move |event, window_target| {
       match event {
           winit::event::Event::WindowEvent { event, .. } => match event {
               WindowEvent::CloseRequested => {
                   window_target.exit();
               },
               WindowEvent::RedrawRequested => {
                   let mut paint = Paint::default();
                   paint.set_color(tiny_skia::Color::BLACK);
                   let rect = tiny_skia::Rect::from_xywh(0.0,0.0,50.0,50.0).unwrap();
                   let path = PathBuilder::from_rect(rect);
                   pixmap.fill_path(&path,&paint,FillRule::Winding,Transform::identity(), None);
                   pixels.frame_mut().copy_from_slice(pixmap.data());
                   pixels.render().unwrap();
               },
               _ => {}
           }
           _ =>{}
       }
    }).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiny_skia(){
        dbg!("Starting");
        render();
    }

    #[tokio::test]
    async fn setup_works() {
        let (_device, _queue) = setup().await;
    }
}
