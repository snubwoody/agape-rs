use helium::{colors::{BLACK, BLUE, RED},};
use ruby::Color;

#[tokio::main]
async fn main() -> ruby::Result<()> {
	unsafe {
		std::env::set_var("RUST_LOG", "warn,helium_renderer=trace");
	}
    env_logger::init();
	
	let app = ruby::App::new()?;

	app.run(|r|{
		r.draw_square(250.0, BLUE, 24.0);
		r.draw_square(250.0, BLUE, 24.0);
		r.draw_square(550.0, Color::rgb(24, 240, 120), 24.0);
		r.draw_square(50.0, BLACK, 24.0);
		r.draw_square(50.0, RED, 24.0);
	}).await?;

	Ok(())
}
