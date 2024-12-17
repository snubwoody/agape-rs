use helium::{app::{events::EventQueue, view::View, App}, hex, hstack, surface::text::TextSurface, vstack, widgets::{Button, Container, Rect, Text, Widget, WidgetBody}, Color, TRANSPARENT};

const BACKGROUND:Color = Color::Hex("#121212");
const GREY:Color = Color::Hex("#414141");
const SPOTIFY_GREEN:Color = Color::Hex("#3be477");

fn main(){
	let event_queue = EventQueue::new();

	let chips = hstack!{
		Chip("Playlist"),
		Chip("Album"),
		Chip("Artist"),
		Chip("Downloaded")
	}.spacing(12);

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
	}.color(BACKGROUND).spacing(24);

	let home_page = View::new(sidepanel, event_queue);
	
	App::new().add_view(home_page).run();
}

struct Chip(&'static str);

impl Widget for Chip {
	fn build(&self) -> helium::widgets::WidgetBody {
		let text = Text::new(&self.0);
		
		Container::new(text)
		.corner_radius(4)
		.color(GREY)
		.padding(12)
		.build()
	}
}