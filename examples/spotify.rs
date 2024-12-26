use helium::{
    app::{events::EventQueue, view::View, App},
    hstack, vstack,
    widgets::{Button, Container, Rect, Text, Widget, WidgetBody},
    Color, Size, TRANSPARENT,
};
use std::env;

const BACKGROUND: Color = Color::Hex("#121212");
const GREY: Color = Color::Hex("#414141");
const SPOTIFY_GREEN: Color = Color::Hex("#3be477");

fn main() {
    env::set_var("RUST_LOG", "trace,wgpu_core=error,naga=warn,wgpu_hal=error");
    env_logger::init();

    let event_queue = EventQueue::new();

    let chips = hstack! {
        Chip("Playlist"),
        Chip("Album"),
        Chip("Artist"),
        Chip("Downloaded")
    }
    .spacing(12);

    let sidepanel = vstack! {
        Text::new("Your library"),
        chips,
        SidebarItem("Liked songs"),
        SidebarItem("Channel Orange"),
        SidebarItem("Wunna"),
        SidebarItem("2014 Forest Hills Drive")
    }
    .color(BACKGROUND)
    .spacing(24);

    let mainpanel = vstack! {
        hstack!{
            Chip("All"),
            Chip("Music"),
            Chip("Podcasts")
        }.spacing(12)
    };

    let home_page = hstack! {sidepanel,mainpanel};
    let home = View::new(home_page, event_queue);

    App::new().add_view(home).run();
}

fn Chip(text: &str) -> impl Widget {
    let text = Text::new(text);

    Container::new(text)
        .corner_radius(4)
        .color(GREY)
        .padding(12)
}

fn SidebarItem(title:&str) -> impl Widget{
	hstack! {
		Rect::new(50.0, 50.0, GREY).corner_radius(12),
		vstack!{
			Text::new(title),
			hstack!{
				Text::new("Playlist"),
				Text::new("Charlemagne")
			}
		}
	}
	.spacing(12)
}

