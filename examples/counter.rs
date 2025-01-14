use helium::{
    app::App,
    hstack,
    page::Page,
    widgets::{icon::feather_icons, Circle, Image, Rect, Text},
    BLACK,
};

fn main() {
    env_logger::init();
    app();
}

fn app() {
    let main = Rect::new(150.0, 150.0, BLACK)
		.corner_radius(24)
		.on_click(||println!("Hello world"));

    let page = Page::new(main);

    App::new().add_page(page).run().unwrap();
}
