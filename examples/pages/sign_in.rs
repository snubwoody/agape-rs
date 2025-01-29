#![allow(non_snake_case)]
use helium::{colors::tailwind_colors, hex, vstack, widgets::*, App, Page, BLACK, WHITE};

fn main() -> Result<(), helium::Error> {
    env_logger::init();
    // TODO export hstack from widgets

    let body = vstack! {
        Form()
    }
    .spacing(24)
    .fill()
    .align_center();

    let page = Page::new(body);

    App::new().add_page(page).run()?;

    Ok(())
}

fn Form() -> impl Widget {
    vstack! {
        Text::new("Sign in"),
        InputField("Email"),
        InputField("Password"),
        Text::new("Forgot password"),
        Button::text("Sign in")
            .color(BLACK)
            .corner_radius(12)
            .font_color(WHITE)
            .fill_width(),
    }
    .align_center()
}

// Turn into widget
fn InputField(label: &str) -> impl Widget {
    vstack! {
        Text::new(label),
        TextField::new()
    }
    .spacing(8)
}
