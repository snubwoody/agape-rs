use helium::{app::{events::EventQueue, view::View, App}, hex, hstack, vstack, widgets::{Button, Container, Rect, Text, Widget}, Color, TRANSPARENT};

const BACKGROUND:Color = Color::Hex("#121212");
const GREY:Color = Color::Hex("#414141");
const SPOTIFY_GREEN:Color = Color::Hex("#3be477");

fn main(){
	let event_queue = EventQueue::new();

	let playlist_chip = Container::new(Text::new("Playlist")).color(GREY).padding(12).corner_radius(4);
	let album_chip = Container::new(Text::new("Album")).color(GREY).padding(12).corner_radius(4);
	let artist_chip = Container::new(Text::new("Artist")).color(GREY).padding(12).corner_radius(4);
	let downloaded_chip = Container::new(Text::new("Downloaded")).color(GREY).padding(12).corner_radius(4);

	let chips = hstack!{playlist_chip,album_chip,artist_chip,downloaded_chip}.spacing(12);
	let sidebar_item = hstack!{
		Rect::new(50, 50, GREY).corner_radius(12),
		vstack!{
			Text::new("Liked songs"),
			hstack!{
				Text::new("Playlist"),
				Text::new("Charlemagne")
			}
		}
	};

	let sidepanel = vstack!{
		chips,
		sidebar_item
	}.color(BACKGROUND);

	let home_page = View::new(sidepanel, event_queue);
	
	App::new().add_view(home_page).run();
}