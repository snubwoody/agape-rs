use rustui::{
    app::{view::View, App},
    colour::{self},
    widgets::Rect,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    new_app().await;
}

async fn new_app() {
    let rect = Rect::new(100.0, 100.0, colour::TEAL);

    let page = View::new(rect);
    let app = App::new().await.add_view(page);
    app.run().await;
}
