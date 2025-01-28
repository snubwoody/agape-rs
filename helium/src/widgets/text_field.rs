use super::{Text, Widget};
use crate::{
    events::Element,
};
use crystal::{BlockLayout, BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use helium_core::color::Color;
use helium_renderer::Rect;

/// Contains editable text
pub struct TextField {
    id: String,
    text: Option<Text>,
    background_color: Color,
	corner_radius:u32
}

impl TextField {
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(),
            text: None,
            background_color: Color::default(),
			corner_radius:12
        }
    }

    pub fn on_click(mut self, f: impl FnMut() + 'static) -> Self {
        self
    }

	pub fn background_color(mut self,background_color:Color) -> Self{
		self.background_color = background_color;
		self
	}

    fn on_input(&mut self) {}
}

impl Widget for TextField {
    fn id(&self) -> &str {
        &self.id
    }

    fn tick(&mut self, elements: &[Element]) {
    }

	fn process_key(&mut self,key:&winit::keyboard::Key) {
		match key {
			winit::keyboard::Key::Character(character) => {
				match &self.text {
					Some(text) => {
						let mut content = String::from(&text.text);
						content.push_str(&character);
						self.text = Some(Text::new(&content))
					},
					None => {
						self.text = Some(Text::new(&character))
					}
				}
			},
			winit::keyboard::Key::Named(named_key) => {
				match named_key {
					winit::keyboard::NamedKey::Backspace => {
						if let Some(text) = &mut self.text {
							if text.text.len() == 1{
								self.text = None;
								return;
							}
							text.text.pop(); // FIXME panics
						};
					},
					winit::keyboard::NamedKey::Space => {
						if let Some(text) = &mut self.text {
							text.text.push_str(" ");
						};
					}
					_ => {}
				}
			},
			_ => {}
		}
	}

    fn layout(&self) -> Box<dyn crystal::Layout> {
		let child = match &self.text {
			Some(text) => text.layout(),
			None => {
				let mut child_layout = EmptyLayout::new();
				child_layout.intrinsic_size.width = BoxSizing::Fixed(200.0);
				child_layout.intrinsic_size.height = BoxSizing::Fixed(40.0);
				Box::new(child_layout)
			}
		};

        let mut layout = BlockLayout::new(child);
		layout.id = self.id.clone();
        Box::new(layout)
    }

	fn children(&self) -> Vec<&dyn Widget> {
		match &self.text {
			Some(text) => vec![text],
			None => vec![]
		}
	}

	fn draw(&self,layout:&dyn crystal::Layout,renderer:&mut helium_renderer::Renderer) {
		
		renderer.draw([ // TODO impl From<Layout>
			Rect::from(layout)
				.color(self.background_color)
				.corner_radius(self.corner_radius as f32)
		]);
		// self.text.draw(&*layout.children()[0], renderer);
	}
}
