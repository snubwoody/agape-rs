use helium::{colors::BLACK, crystal::AxisAlignment, vstack, widgets::{Rect, Text}, App};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    // TODO Overflowing widgets will need to be clipped
    let list = vstack! {
        Rect::new(200.0, 200.0).color(BLACK),
        Rect::new(200.0, 200.0).color(BLACK),
        Rect::new(200.0, 200.0).color(BLACK),
        Rect::new(200.0, 200.0).color(BLACK),
        Rect::new(200.0, 200.0).color(BLACK),
    }
    .spacing(24)
    .fill()
	.scrollable() // Enable scrolling
	.cross_axis_alignment(AxisAlignment::Center);

    let mut app = App::new();
    app.add_page(list);
    app.run().await
}
