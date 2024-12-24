use std::{env, fs, io::{self, Write}};

use helium::{app::{events::EventQueue, view::View, App}, hstack, vstack, widgets::{Button, Container, Rect, Text, Widget, WidgetBody}, Color, Size, TRANSPARENT};

const BACKGROUND:Color = Color::Hex("#121212");
const GREY:Color = Color::Hex("#414141");
const SPOTIFY_GREEN:Color = Color::Hex("#3be477");

fn main(){
	env::set_var("RUST_LOG", "trace,wgpu_core=error,naga=warn,wgpu_hal=error");
	env_logger::init();

	let event_queue = EventQueue::new();

	let chips = hstack!{
		Chip("Playlist"),
		Chip("Album"),
		Chip("Artist"),
		Chip("Downloaded")
	}.spacing(12);

	let sidepanel = vstack!{
		Text::new("Your library"),
		chips,
		SidebarItem::new("Liked songs"),
		SidebarItem::new("Channel Orange"),
		SidebarItem::new("Wunna"),
		SidebarItem::new("2014 Forest Hills Drive")
	}.color(BACKGROUND).spacing(24);

	let mainpanel = vstack!{
		hstack!{
			Chip("All"),
			Chip("Music"),
			Chip("Podcasts")
		}.spacing(12)
	};

	let home_page = hstack!{sidepanel,mainpanel};
	let home = View::new(home_page, event_queue);
	
	App::new().add_view(home).run();
}

// struct Chip(&'static str);

// impl Widget for Chip {
// 	fn build(&self) -> helium::widgets::WidgetBody {
// 		let text = Text::new(&self.0);
		
// 		Container::new(text)
// 		.corner_radius(4)
// 		.color(GREY)
// 		.padding(12)
// 		.build()
// 		.label(&self.0)
// 	}
// }

// struct SidebarItem{
// 	title:String
// }

// impl SidebarItem {
// 	fn new(title:&str) -> Self{
// 		Self{
// 			title:title.into()
// 		}
// 	}
// }

// impl Widget for SidebarItem {
// 	fn build(&self) -> WidgetBody {
// 		hstack!{
// 			Rect::new(50, 50, GREY).corner_radius(12),
// 			vstack!{
// 				Text::new(&self.title),
// 				hstack!{
// 					Text::new("Playlist"),
// 					Text::new("Charlemagne")
// 				}
// 			}
// 		}.spacing(12).build()		
// 	}
// }