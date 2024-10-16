use helium::{
    app::{view::View, App}, 
	colour::{self}, 
	hstack, 
	widgets::Rect
};

fn main() {
    env_logger::init();
    new_app()
}

fn new_app() {
    let rect = Rect::new(100.0, 100.0, colour::TEAL);
    let rect2 = Rect::new(100.0, 100.0, colour::PINK);
    let rect3 = Rect::new(100.0, 100.0, colour::INDIGO);

	let hstack = hstack![rect,rect2,rect3].spacing(20);

    let page = View::new(hstack);
    let app = App::new().add_view(page);
    app.run();
}
