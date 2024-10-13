use rustui::{
    app::{view::View, App}, colour::{BLACK, BLUE, GREEN}, hstack, vstack, widgets::{button::Button, rect::Rect, stack::Stack, text::Text}
};

#[tokio::main]
async fn main() {
	env_logger::init();
    new_app().await;
}

async fn new_app() {
	let rect = Rect::new(200.0, 200.0, rustui::colour::Colour::Rgb(22, 21, 24));

    let page = View::new(rect);
    let app = App::new().await.add_view(page);
    app.run().await;
}
