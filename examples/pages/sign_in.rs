use helium::{
	colors::tailwind_colors, 
	events::EventContext, 
	hex, 
	vstack, 
	widgets::*, 
	App, 
	Page, 
};

fn main(){
	let mut cx = EventContext::new();
	// TODO export hstack from widgets

	let form = vstack!{
		Text::new("Sign in"),
		InputField("Email"),
		InputField("Password"),
		Text::new("Forgot password"),
		Button::new(Text::new("Hello world"))
			.color(hex!("#000000")),
		TextField::new(),
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

// Turn into widget
fn InputField(label:&str) -> impl Widget{
	vstack! {
		Text::new(label),
		Rect::new(120.0,20.0).color(
			tailwind_colors::NEUTRAL300
		).corner_radius(12),
	}
	.spacing(8)
}