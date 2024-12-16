use helium::{app::{events::EventQueue, view::View, App}, hex, hstack, vstack, widgets::{Button, Container, Rect, Text}, Color};

const BACKGROUND:Color = Color::Hex("#121212");
const GREY:Color = Color::Hex("#414141");
const SPOTIFY_GREEN:Color = Color::Hex("#3be477");

fn main(){
	let event_queue = EventQueue::new();
	let button = Button::new("Click me").color(hex!("#000000"));
	let chip = Container::new(Text::new("Playlist"));
	let sidepanel = vstack!{
		chip
	};


	let home_page = View::new(sidepanel, event_queue);
	
	App::new().add_view(home_page).run();
}