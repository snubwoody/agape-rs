use helium::{
    app::{events::EventQueue, view::View, App},
    hstack,
    widgets::{Circle, Rect},
    BLACK,
};

fn main() {
    env_logger::init();
    app();
}

fn app() {
    let event_loop = EventQueue::new();

    let main = hstack! {
       Rect::new(150.0,150.0,BLACK)
    }
    .spacing(54);

    let page = View::new(main, event_loop);

    App::new().add_view(page).run();
}
