mod widgets;
mod view;
mod colour;
mod app;
pub mod surface;
pub mod text;
pub mod vertex;
use widgets::text::Text;
use crate::surface::Surface;
use crate::widgets::Widget;
use crate::view::View;
use crate::app::App;
#[macro_use]
extern crate glium;


fn main() {
	run_app();
}

fn run_app<'a>() {
	let text = Text::new(40, 50, "Click me", "#000", 20);

	let page = View{
		child:text
	};

	let app = 
		App::new()
		.add_view(page);

	app.run()
}



/* struct EventManager {
    event_queue: VecDeque<WidgetEvent>,
}

impl EventManager {
    fn new() -> Self {
        EventManager {
            event_queue: VecDeque::new(),
        }
    }

    fn push_event(&mut self, event: WidgetEvent) {
        self.event_queue.push_back(event);
    }

    fn process_events(&mut self) {
        while let Some(event) = self.event_queue.pop_front() {
            // Process the event
        }
    }
} */


/// Map value from one range to another. Any overflow is clipped to the min or max
fn map(mut value:f32,input_range:[f32;2],output_range:[f32;2]) -> f32{
	if value > input_range[1]{
		value = input_range[1]
	}
	else if value < input_range[0] {
		value = input_range[0]
	}

	let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
	let offset = input_range[0]*(scale)+output_range[0];

	return  value * scale + offset;
}

