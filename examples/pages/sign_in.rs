#![allow(non_snake_case)]
use helium::{colors::tailwind_colors, vstack, widgets::*, App, Page, colors::{BLACK, WHITE}};

#[tokio::main]
async fn main() -> Result<(), helium::Error> {
    // TODO add underlines
    env_logger::init();
    // TODO export hstack from widgets

    let body = vstack! {
        Form()
    }
    .fill()
    .align_center();

    let mut app = App::new();
    app.add_page(body);
    app.run().await?;

    Ok(())
}

fn Form() -> impl Widget {
    vstack! {
        Text::new("Sign in")
            .font_size(24),
        InputField("Email"),
        InputField("Password"),
        Button::text("Sign in")
            .color(BLACK)
            .font_color(tailwind_colors::NEUTRAL200)
            .fill_width()
            .on_click(||{
                let num = 15.0;
            })
            .padding(12),
        Text::new("Forgot password?"),
    }
    .spacing(36)
    .align_center()
}

// Turn into widget
fn InputField(label: &str) -> impl Widget {
    vstack! {
        Text::new(label),
        TextField::new()
            .fixed_height(40.0)
            .fixed_width(400.0),
    }
    .spacing(12)
}
