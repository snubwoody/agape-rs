use helium::{
	hstack, 
	widgets::{icon::feather_icons, *}, 
	State,
	App
};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    std::env::set_var("RUST_LOG", "warn,helium=trace");
    env_logger::init();

	let mut count = 1;
	
	let main = hstack! {
		Button::new(feather_icons::minus()),
		Text::new(&format!("{count}")),
		Button::new(feather_icons::plus()),
    }
    .spacing(24)
    .fill()
	.align_center();

    let mut app = App::new();
    app.add_page(main);

    app.run().await?;

    Ok(())
}
