use std::{cell::Cell, sync::Arc};

use helium::{
	crystal::HorizontalLayout, 
	hstack, 
	nanoid, 
	renderer::Renderer, 
	widgets::{icon::feather_icons::*, *}, 
	App
};


struct Counter{
	id: String,
	count: Arc<Cell<i32>>
}

impl Counter {
	fn new() -> Self{
		Self {
			id: nanoid!(), 
			count: Arc::new(Cell::new(1))
		}
	}

	pub fn increment(&self){
		let val = self.count.get();
		self.count.replace(val + 1);
	}
	
	pub fn decrement(&self){
		let val = self.count.get();
		self.count.replace(val - 1);
	}

	fn build(&mut self){
		let count = self.count.clone();
		hstack! {
			Button::new(plus()).on_click(move ||{
				count.replace(23);
			})
		};
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
