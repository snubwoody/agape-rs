use helium::colors::BLACK;
use ruby::{Color, RectSurface};


#[tokio::main]
async fn main() -> ruby::Result<()> {
	unsafe {
		std::env::set_var("RUST_LOG", "warn,helium_renderer=trace");
	}
    env_logger::init();
	
	let app = ruby::App::new()?;

	app.run(|r|{
		r.draw_square(250.0, Color::rgb(25, 223, 102), 24.0);
		r.draw_square(50.0, Color::rgb(205, 223, 102), 2.0);
		r.draw_square(550.0, Color::rgb(0, 223, 252), 2.0);
	}).await?;

	Ok(())
}
