use helium::{Position, BLACK};
use helium_renderer::{Circle, Icon, Image, Rect, Renderer, Text};
use winit::{event::WindowEvent, event_loop::EventLoop, window::WindowBuilder};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace,naga=warn,wgpu_core=warn");
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = Renderer::new(&window).await;

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
                    renderer.draw([Rect::new(200.0, 200.0).color(BLACK).corner_radius(24.0)]);
                    renderer.draw([Circle::new(120.0).color(BLACK).position(150.0, 200.0)]);
                    renderer.draw([Text::new("Hello world").position(350.0, 0.0)]);
                    renderer.draw([Icon::new(
                        image::load_from_memory(include_bytes!("./temp/legends never die.png"))
                            .unwrap(),
                    )
                    .position(450.0, 150.0)]);
                    renderer.render();
                }
                event => {
                    window.request_redraw();
                }
            },
            _ => {}
        })
        .expect("Event loop error occured");
}
