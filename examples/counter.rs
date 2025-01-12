use helium::{
    app::App,
    hstack,
    page::Page,
    widgets::{icon::feather_icons, Circle, Rect,Text},
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
	   Text::new("Hello world")
       //feather_icons::search()
    }
    .spacing(54);

    let page = Page::new(main);

    App::new().add_view(page).run();
}
