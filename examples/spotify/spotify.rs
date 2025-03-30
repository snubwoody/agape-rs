#![allow(non_snake_case)]
use helium::{
    colors::{BLACK, WHITE}, crystal::AxisAlignment, hex, hstack, vstack, widgets::*, App, Color, Rgba
};
use icon::feather_icons;

const BACKGROUND: Color<Rgba> = Color::rgb(0, 0,0);
const GREY: Color<Rgba> = Color::rgb(40, 40, 40);
const SPOTIFY_GREEN: Color<Rgba> = Color::rgb(0, 255, 0);

// TODO theres some sizes that are making the icons pixelated, very weird
#[tokio::main]
async fn main() -> Result<(), helium::Error> {
	unsafe {
		std::env::set_var("RUST_LOG", "warn,helium_renderer=trace");
	}
    env_logger::init();

    let announcements = Rect::new(0.0, 400.0)
        .color(BACKGROUND)
        .flex_width(1)
        .corner_radius(24);

    let mainpanel = vstack! {
        announcements,
        hstack!{
            Chip("All"),
            Chip("Music"),
            Chip("Podcasts")
        }.spacing(12),
        hstack!{
            HomePlaylist("Car music"),
            HomePlaylist("Euphoria"),
            HomePlaylist("Liked songs"),
            HomePlaylist("Hype hype"),
        }
        .spacing(24)
        .fill_width(),
        hstack!{
            HomePlaylist("Car music"),
            HomePlaylist("Euphoria"),
            HomePlaylist("Liked songs"),
            HomePlaylist("Hype hype"),
        }
        .spacing(24)
        .fill_width(),
        HomeSection(),
        HomeSection(),
        HomeSection(),
        HomeSection(),
        HomeSection(),
    }
    .padding(24)
    .spacing(24)
	.scrollable()
    .fill_width();

    let home_page = hstack! {Sidebar(),mainpanel}
        .fill_width()
        .fill_height()
        .padding(12);

    let home = vstack! {
        Navbar(),
        home_page,
        BottomBar()
    }
    .color(BLACK)
    .fill_height()
    .fill_width();

    let mut app = App::new();
    app.add_page(home);
    app.run().await?;

    Ok(())
}

fn Sidebar() -> impl Widget {
    let chips = hstack! {
        Chip("Playlist"),
        Chip("Album"),
        Chip("Artist"),
        Chip("Downloaded")
    }
    .spacing(12)
    .fill_width();

    vstack! {
        hstack!{
            feather_icons::menu().color(WHITE),
            Text::new("Your library").color(WHITE),
            Spacer::new(),
            feather_icons::plus().color(WHITE),
            feather_icons::arrow_right().color(WHITE)
        }
        .cross_axis_alignment(AxisAlignment::Center)
        .fill_width()
        .spacing(12),
        chips,
        hstack!{
            feather_icons::search().color(WHITE),
            Spacer::new(),
            Text::new("Recents").color(WHITE),
            feather_icons::list().color(WHITE),
        }
        .fill_width()
        .cross_axis_alignment(AxisAlignment::Center),
        SidebarPlaylist("Liked songs"),
        SidebarPlaylist("Channel Orange"),
        SidebarPlaylist("Wunna"),
        SidebarPlaylist("2014 Forest Hills Drive")
    }
    .spacing(24)
    .padding(24)
    .corner_radius(24)
    .fill_height()
    .color(BACKGROUND)
}

fn DiscoveryPlaylist() -> impl Widget {
    vstack! {
        Image::bytes(include_bytes!("COLOURS - PARTYNEXTDOOR.jpg"))
            .unwrap()
            .fixed_width(150.0)
            .fixed_height(150.0),
        Text::new("Daily mix")
    }
    .spacing(12)
}

fn HomeSection() -> impl Widget {
    vstack! {
        hstack!{
            Text::new("Made for charlemagne")
            .font_size(24)
            .color(WHITE),
            Spacer::new(),
            Text::new("Show all").color(WHITE),
        }
        .fill_width(),
        hstack!{
            DiscoveryPlaylist(),
            DiscoveryPlaylist(),
            DiscoveryPlaylist(),
            DiscoveryPlaylist(),
            DiscoveryPlaylist(),
            DiscoveryPlaylist(),
        }.spacing(36)

    }
    .spacing(10)
    .fill_width()
}

fn BottomBar() -> impl Widget {
    hstack! {
        hstack!{
            Image::bytes(include_bytes!("COLOURS - PARTYNEXTDOOR.jpg"))
            .unwrap()
            .fixed_width(50.0)
            .fixed_height(50.0),
            vstack!{
                Text::new("You've been missed")
                .color(WHITE),
                Text::new("PARTYNEXTDOOR")
                .color(WHITE)
            }
            .spacing(4)
        }
        .spacing(12)
        .cross_axis_alignment(AxisAlignment::Center),
        Spacer::new(),
        vstack!{
            hstack!{
                feather_icons::shuffle().color(WHITE),
                feather_icons::skip_back().color(WHITE),
                feather_icons::play().color(WHITE),
                feather_icons::skip_forward().color(WHITE),
                feather_icons::repeat().color(WHITE),
                Text::new("Placeholder")
            }
            .spacing(16),
            hstack!{
                Text::new("0:00")
                .color(WHITE),
                Rect::new(350.0, 5.0)
                .color(BLACK)
                .corner_radius(2),
                Text::new("4:00")
                .color(WHITE)
            }
            .spacing(8)
            .cross_axis_alignment(AxisAlignment::Center)
        }
        .spacing(12)
        .fit_height()
        .cross_axis_alignment(AxisAlignment::Center),
        Spacer::new(),
    }
    .fill_width()
    .padding(16)
    .color(BLACK)
}

fn Navbar() -> impl Widget {
    hstack! {
        feather_icons::more_horizontal().color(WHITE),
        feather_icons::chevron_left().color(WHITE),
        feather_icons::chevron_right().color(WHITE),
        Spacer::new(),
        feather_icons::home().color(WHITE),
        feather_icons::search().color(WHITE),
        Text::new("What do you want to play?")
        .color(WHITE),
        Spacer::new(),
        feather_icons::bell().color(WHITE),
        feather_icons::users().color(WHITE)
    }
    .fill_width()
    .spacing(12)
    .padding(12)
    .cross_axis_alignment(AxisAlignment::Center)
    .color(BLACK)
}

fn Chip(text: &str) -> impl Widget {
    let text = Text::new(text).color(WHITE);

    Container::new(text)
        .corner_radius(14)
        .color(GREY)
        .padding(12)
}

fn SidebarPlaylist(title: &str) -> impl Widget {
    hstack! {
        Image::bytes(include_bytes!("COLOURS - PARTYNEXTDOOR.jpg"))
        .unwrap()
        .fixed_width(50.0)
        .fixed_height(50.0),
        vstack!{
            Text::new(title).color(WHITE).font_size(20),
            hstack!{
                Text::new("Playlist").color(WHITE),
                Text::new("Charlemagne").color(WHITE)
            }
            .spacing(12)
        }
        .spacing(8)
    }
    .spacing(12)
    .cross_axis_alignment(AxisAlignment::Center)
}

fn HomePlaylist(name: &str) -> impl Widget {
    hstack! {
        Image::bytes(include_bytes!("COLOURS - PARTYNEXTDOOR.jpg"))
        .unwrap()
        .fixed_width(50.0)
        .fixed_height(50.0),
        Text::new(name).color(WHITE)
    }
    .spacing(12)
    .cross_axis_alignment(AxisAlignment::Center)
    .fill_width()
}
