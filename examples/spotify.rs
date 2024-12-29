use helium::{
    app::{events::EventQueue, view::View, App}, hstack, vstack, widgets::{icon::feather_icons, Button, Container, Image, Rect, Spacer, Text, Widget, WidgetBody}, Color, LayoutSolver, Size, BLACK, TRANSPARENT
};
use std::{env, fs::OpenOptions, io::{BufWriter, Write}};

const BACKGROUND: Color = Color::Hex("#121212");
const GREY: Color = Color::Hex("#414141");
const SPOTIFY_GREEN: Color = Color::Hex("#3be477");

fn main() {
	// TODO add individual padding
	// TODO add alignment options
    env::set_var("RUST_LOG", "trace,wgpu_core=error,naga=warn,wgpu_hal=error");
    env_logger::init();

    let event_queue = EventQueue::new();

	let announcements = Rect::new(0.0, 400.0, BACKGROUND).flex_width(1).corner_radius(24);

    let chips = hstack! {
        Chip("Playlist"),
        Chip("Album"),
        Chip("Artist"),
        Chip("Downloaded")
    }
    .spacing(12)
	.width_fit();

	// FIXME has a width of 0 should be like 500
    let sidebar = vstack! {
		hstack!{
			feather_icons::menu(),
			Text::new("Your library"),
			feather_icons::plus(),
			feather_icons::arrow_right()
		},
        chips,
		hstack!{
			feather_icons::search(),
			Text::new("Recents"),
			feather_icons::list()
		},
        SidebarItem("Liked songs"),
        SidebarItem("Channel Orange"),
        SidebarItem("Wunna"),
        SidebarItem("2014 Forest Hills Drive")
    }
    .color(BACKGROUND)
    .spacing(24)
	.padding(24)
	.height_fill();

    let mainpanel = vstack!{
		announcements,
        hstack!{
            Chip("All"),
            Chip("Music"),
            Chip("Podcasts")
        }.spacing(12)
    }
	.padding(24)
	.spacing(24)
	.width_fill();

    let home_page = hstack!{sidebar,mainpanel}
		.width_fill()
		.height_fill();

	let home = vstack!{
		Navbar(),
		home_page,
		BottomBar()
	}
	.height_fill()
	.width_fill();

	let (_,mut layout) = home.build();
	LayoutSolver::solve(&mut *layout, Size::new(500.0, 500.0));

	// TODO im probably going to be using this a lot so probs just make it a function
	// maybe LayoutSolver::solve_and_write(path:&str)
	let file = OpenOptions::new()
		.write(true)
		.read(true)
		.open("C:/Users/wakun/Projects/Tools/Rust-UI/examples/temp/layout.txt").unwrap();
	let mut writer = BufWriter::new(file);

	writer.write(format!("{:#?}",layout).as_bytes()).unwrap();
	writer.flush().unwrap();
	
	let home = View::new(home, event_queue);
    App::new().add_view(home).run();

}

fn BottomBar() -> impl Widget{
	hstack!{
		hstack!{
			Rect::new(50.0, 50.0, BLACK)
			.corner_radius(12),
			vstack!{
				Text::new("You've been missed"),
				Text::new("PARTYNEXTDOOR")
			}
		},
		Spacer(),
		vstack!{
			hstack!{
				feather_icons::shuffle(),
				feather_icons::skip_back(),
				feather_icons::play(),
				feather_icons::skip_forward(),
				feather_icons::repeat()
			},
			hstack!{
				Text::new("0:00"),
				Rect::new(150.0, 5.0, BLACK).corner_radius(2),
				Text::new("4:00")
			}
		}.height_fit()
	}.width_fill()
}

fn Navbar() -> impl Widget{
	hstack!{
		Text::new("Test")
	}
	.width_fill()
	.color(BACKGROUND)
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

