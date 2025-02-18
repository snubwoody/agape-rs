use helium::{
	colors::BLACK, hex, hstack, nanoid, renderer::Renderer, widgets::{icon::feather_icons::*, *}, App
};


#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    std::env::set_var("RUST_LOG", "error,helium=trace");
    env_logger::init();

    let mut app = App::new();

    app.run().await?;

    Ok(())
}
