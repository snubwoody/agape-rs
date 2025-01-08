use helium::{
    app::{events::EventQueue, view::View, App},
    hstack,
    widgets::Circle,
    BLACK,
};

fn main() {
    env_logger::init();
    app();
}

fn app() {
    let event_loop = EventQueue::new();

    let main = hstack! {
        Circle::new(100, BLACK),
        Circle::new(100, BLACK),
        Circle::new(100, BLACK),
        Circle::new(100, BLACK),
        Circle::new(100, BLACK),
    }
    .spacing(54);

    let page = View::new(main, event_loop);

    App::new().add_view(page).run();
}
