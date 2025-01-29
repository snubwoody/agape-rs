use helium::{widgets::*, App, Page, BLACK};

fn main() {
    env_logger::init();
    app();
}

fn app() {
    let main = Rect::new(150.0, 150.0).color(BLACK).corner_radius(24);

    let page = Page::new(main);

    App::new().add_page(page).run().unwrap();
}
