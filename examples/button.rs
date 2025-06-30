use helium::widgets::{Button, Text};
use helium::{App, hex};

fn main() {
    let button = Button::new(Text::new("Click me"))
        .on_click(|| println!("I have been clicked!"))
        .on_hover(|| println!("Get your cursor off me!"))
        .color(hex!("#000000"));

    let app = App::new(button);
    app.run().unwrap()
}
