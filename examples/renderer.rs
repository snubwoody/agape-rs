use std::time::Instant;

use helium_renderer::{Image, Renderer, Text};
use image::{DynamicImage, ImageBuffer, Rgba};
use winit::{event::WindowEvent, event_loop::EventLoop, window::WindowBuilder};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warn,helium_renderer=trace");
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = Renderer::new(&window).await;

    let image =
        image::load_from_memory(include_bytes!("spotify/COLOURS - PARTYNEXTDOOR.jpg")).unwrap();
    let data = image.to_rgba8();

    event_loop
        .run(|event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size.into());
                }
                WindowEvent::RedrawRequested => draw(data.clone(), &mut renderer),
                _ => {
                    window.request_redraw();
                }
            },
            _ => {}
        })
        .expect("Event loop error occured");
}

fn draw(data: ImageBuffer<Rgba<u8>, Vec<u8>>, renderer: &mut Renderer) {
    let instant = Instant::now();

    renderer.draw([
        Image::new(data.clone()).size(250.0, 250.0),
        Image::new(data.clone())
            .size(250.0, 250.0)
            .position(300.0, 0.0),
        Image::new(data.clone())
            .size(250.0, 250.0)
            .position(0.0, 300.0),
    ]);

    renderer.render();
    println!("Draw call {:?}", instant.elapsed())
}
