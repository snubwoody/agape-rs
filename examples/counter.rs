use helium::{app::App, events::EventContext, page::Page, widgets::*, BLACK};

fn main() {
    env_logger::init();
    app();
}

fn app() {
    let mut cx = EventContext::new();

    let main = Rect::new(150.0, 150.0)
        .color(BLACK)
        .corner_radius(24)
        .on_hover(&mut cx, || println!("I was hovered"))
        .on_click(&mut cx, || println!("I was clicked"));

    let page = Page::new(cx, main);

    App::new().add_page(page).run().unwrap();
}
