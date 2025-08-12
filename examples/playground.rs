// Playground example, not intended for serious use.
use agape::{App, widgets::*};

fn main() {
    App::new(TextBox::new("Hi")).run().unwrap()
}

struct TextBox {
    text: String,
}

impl TextBox {
    pub fn new(text: &str) -> TextBox {
        Self {
            text: String::from(text),
        }
    }
}

impl View for TextBox {
    fn update(&mut self) {
        self.text.push('a');
    }

    fn view(&self) -> Box<dyn Widget> {
        Box::new(Text::new(self.text.as_str()))
    }
}
