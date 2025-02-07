use helium::{crystal::AxisAlignment, vstack, widgets::*, App, BLACK};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    std::env::set_var("RUST_LOG", "warn,helium=trace");
    env_logger::init();
	// Add list view
	
    let main = vstack! {
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
        Rect::new(200.0, 50.0).color(BLACK),
    }
	.spacing(24)
    .fill()
    .cross_axis_alignment(AxisAlignment::Center);

    let mut app = App::new();
    app.add_page(main);

    app.run().await?;

    Ok(())
}
