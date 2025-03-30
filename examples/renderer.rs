use helium::colors::BLACK;
use ruby::RectSurface;


#[tokio::main]
async fn main() -> ruby::Result<()> {
	unsafe {
		std::env::set_var("RUST_LOG", "warn,helium_renderer=trace");
	}
    env_logger::init();
	
	let app = ruby::App::new()?;

	app.run(|r|{
		r.draw([RectSurface::new(300.0, 200.0).color(BLACK)]);
	}).await?;

	Ok(())
}
