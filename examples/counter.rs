use helium::{widgets::*, App, Page, BLACK};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    env_logger::init();
    let main = Rect::new(150.0, 150.0).color(BLACK).corner_radius(24);
	//let image = Await::new(future, pending)
    let mut app = App::new();
    app.add_page(main);

    app.run().await?;

    Ok(())
}


async fn load_data(){
	let url = "";
}