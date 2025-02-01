use std::time::Duration;

use helium::{hstack, widgets::*, App};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    std::env::set_var("RUST_LOG", "warn,helium=trace");
	env_logger::init();
	let load_data = async {
		tokio::time::sleep(Duration::from_secs(6)).await;
		Text::new("Loaded")
	};
	let main = hstack! {
		Await::new(load_data, Text::new("Loading"))
	}
	.fill()
	.align_center();
    
	let mut app = App::new();
    app.add_page(main);

    app.run().await?;

    Ok(())
}
