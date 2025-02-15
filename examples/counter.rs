use helium::{
	hstack, 
	widgets::{icon::feather_icons::*, *}, 
	State,
	App
};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    std::env::set_var("RUST_LOG", "warn,helium=trace");
    env_logger::init();

	let count = 1;
	
	let main = hstack! {
		Button::new(minus()),
		Text::new(&format!("{count}")),
		Button::new(plus())
		.on_click(||{
			//count.update(|value| value+=1 ) Increment the count here
		}),
    }
    .spacing(24)
    .fill()
	.align_center();

    let mut app = App::new();
    app.add_page(main);

    app.run().await?;

    Ok(())
}
