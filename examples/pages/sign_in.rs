use helium::{
	events::EventContext, hex, hstack, vstack, widgets::*, App, Page 
};

fn main(){
	let cx = EventContext::new();
	// TODO export hstack from widgets

	let form = vstack! {
		Text::new("Sign in"),
		Text::new("Email"),
		Text::new("Password"),
		Text::new("Forgot password"),
		Button::new(Text::new("Hello world"))
			.color(hex!("#000000")),
	}
	.spacing(24)
	.fill()
	.align_center();

	let page = Page::new(cx, form);

	App::new()
		.add_page(page)
		.run()
		.unwrap();
}