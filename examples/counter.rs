use helium::{widgets::*, App, Page, BLACK};

#[tokio::main]
async fn main() -> Result<(),helium::Error> {
	env_logger::init();
    let main = Rect::new(150.0, 150.0).color(BLACK).corner_radius(24);

    let mut app = App::new();
	app.add_page(main);
	
	app.run().await?;

	Ok(())
}
