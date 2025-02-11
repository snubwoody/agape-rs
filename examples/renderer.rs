use std::time::Instant;

use helium_renderer::{Image, Renderer, Text};
use image::{load_from_memory, DynamicImage, ImageBuffer, Rgba};
use winit::{event::WindowEvent, event_loop::EventLoop, window::WindowBuilder};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warn,helium_renderer=trace");
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = Renderer::new(&window).await;

    let image1 = load_from_memory(include_bytes!("spotify/COLOURS - PARTYNEXTDOOR.jpg")).unwrap();

    let image2 = load_from_memory(include_bytes!("spotify/Drake_-_So_Far_Gone_cover.png")).unwrap();

    let images = [
        Image::new(image1.to_rgba8()).size(250.0, 250.0),
        Image::new(image2.to_rgba8())
            .size(250.0, 250.0)
            .position(300.0, 0.0),
        Image::new(image2.to_rgba8())
            .size(250.0, 250.0)
            .position(0.0, 300.0),
    ];

    event_loop
        .run(|event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size.into());
                }
                WindowEvent::RedrawRequested => {
                    let instant = Instant::now();

                    renderer.draw(images.clone());
                    renderer.render();
                    println!("Draw call {:?}", instant.elapsed())
                }
                _ => {
                    window.request_redraw();
                }
            },
            _ => {}
        })
        .expect("Event loop error occured");
}
