use helium::{
    app::{view::View, App},
    hstack,
    widgets::Rect,
    BLACK,
};

fn main() {
    env_logger::init();
    app();
}

fn app() {
    let main = hstack! {
       Rect::new(150.0,150.0,BLACK),
       Rect::new(150.0,150.0,BLACK),
       Rect::new(150.0,150.0,BLACK),
       Rect::new(150.0,150.0,BLACK),
    }
    .spacing(54);

    let page = View::new(main);

    App::new().add_view(page).run();
}
