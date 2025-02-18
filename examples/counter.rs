use helium::{
	colors::BLACK, crystal::HorizontalLayout, hex, hstack, nanoid, renderer::Renderer, widgets::{icon::feather_icons::*, *}, App
};


struct Counter{
	id: String,
	count: i32
}

impl Counter {
	fn new() -> Self{
		Self {
			id: nanoid!(), 
			count:0 
		}
	}

	pub fn increment(&mut self){
		self.count += 1
	}

	pub fn decrement(&mut self){
		self.count -= 1
	}
}

impl Widget for Counter {
	fn id(&self) -> &str {
		&self.id
	}

	fn layout(&self, renderer: &mut Renderer) -> Box<dyn helium::crystal::Layout> {
		Box::new(HorizontalLayout::default())	
	}

	fn draw(&self, layout: &dyn helium::crystal::Layout, renderer: &mut Renderer) {
		Text::new(&format!("{}",&self.count));
	}
}

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    std::env::set_var("RUST_LOG", "error,helium=trace");
    env_logger::init();

	let counter = Counter::new();
    let mut app = App::new();
	app.add_page(counter);

    app.run().await?;

    Ok(())
}
