use helium::{
    app::{events::EventQueue, view::View, App}, hstack, vstack, widgets::{Button, Container, Rect, Text, Widget, WidgetBody}, Color, LayoutSolver, Size, TRANSPARENT
};
use std::{env, fs::{File, OpenOptions}, io::{BufWriter, Write}};

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
    .spacing(12)
	.width_fit();

	// FIXME has a width of 106 should be like 500
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

	// FIXME has a height of zero
    let home_page = hstack! {sidepanel,mainpanel};

	let (_,mut layout) = home_page.build();
	LayoutSolver::solve(&mut *layout, Size::new(500.0, 500.0));

	let file = OpenOptions::new()
		.write(true)
		.read(true)
		.open("C:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/layout.txt").unwrap();
	let mut writer = BufWriter::new(file);

	writer.write(format!("{:#?}",layout).as_bytes()).unwrap();
	writer.flush().unwrap();
	
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
			}.spacing(12)
		}
	}
	.spacing(12)
}

