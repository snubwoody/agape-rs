use helium::{colors::BLACK, crystal::AxisAlignment, vstack, widgets::{Rect, Text}, App};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    let rect = Rect::new(200.0, 200.0);
    let mut app = App::new(rect);
    app.run().await
}
