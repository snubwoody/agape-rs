use helium::{
    app::App,
    hstack,
    page::Page,
    widgets::*,
    BLACK,
};

fn main() {
    env_logger::init();
    app();
}

fn app() {
    let main = Rect::new(150.0, 150.0, BLACK)
		.corner_radius(24)
		.on_hover(||println!(""))
		.on_tap(||println!("I was tapped"));

    let page = Page::new(main);

    App::new().add_page(page).run().unwrap();
}
