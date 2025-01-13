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
    let main = hstack! {
       Rect::new(150.0,150.0,BLACK),
       Circle::new(100,BLACK),
       Text::new("Hello world"),
       feather_icons::search(),
       feather_icons::home(),
       Image::url("https://upload.wikimedia.org/wikipedia/en/9/93/Kendrick_Lamar_-_GNX.png")
    }
    .spacing(32);

    let page = Page::new(main);

    App::new().add_page(page).run().unwrap();
}
