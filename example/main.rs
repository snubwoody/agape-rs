use helium::{
    app::{view::View, App}, hex, hstack, vstack, widgets::{Button, Rect, Text, Widget}
};

fn main() {
    env_logger::init();
	app();
}

fn app(){
	// FIXME somehow has a block layout
	let color = hex!("#afffff");
	let hstack = vstack![
		Rect::new(200, 200, color.clone()),
		Rect::new(200, 200, color.clone()),
		Rect::new(20,200,color)
	].spacing(54).padding(24).color(hex!("#000000"));

	let page = View::new(hstack);
    let app = App::new().add_view(page);
    
	app.run();
}