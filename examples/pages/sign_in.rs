use helium::{colors::tailwind_colors, hex, vstack, widgets::*, App, Page};

fn main() {
	env_logger::init();
    // TODO export hstack from widgets

    let form = vstack! {
        Text::new("Sign in"),
        InputField("Email"),
        InputField("Password"),
        Text::new("Forgot password"),
        Button::new(Text::new("Sign in"))
            .color(tailwind_colors::NEUTRAL200)
			.corner_radius(12),
        
    }
    .spacing(24)
    .fill()
    .align_center();


    let page = Page::new(form);

    App::new().add_page(page).run().unwrap();
}

// Turn into widget
fn InputField(label: &str) -> impl Widget {
    vstack! {
        Text::new(label),
		TextField::new()
			.background_color(tailwind_colors::NEUTRAL200),
    }
    .spacing(8)
}
