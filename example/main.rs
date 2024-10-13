use rustui::{
    app::{view::View, App}, colour::{self, Colour}, hstack, widgets::{button::Button, rect::Rect, stack::Stack, text::Text}
};

#[tokio::main]
async fn main() {
    env_logger::init();
    new_app().await;
}

async fn new_app() {
    let rect = Rect::new(100.0, 100.0, colour::TEAL);
    let rect2 = Rect::new(100.0, 100.0, colour::TEAL);

	let hstack = hstack![rect,rect2].spacing(200);


    let page = View::new(hstack);
    let app = App::new().await.add_view(page);
    app.run().await;
}
