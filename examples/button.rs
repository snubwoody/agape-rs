use helium::widgets::{Button, Text};
use helium::{App, Color};

fn main() {
    let _ = dotenv::dotenv();
    let button = Button::new(Text::new("Click me"))
        .on_click(|| println!("I have been clicked!"))
        .on_hover(|| println!("Get your cursor off me!"))
        .color(Color::rgb(124, 124, 254));

    let app = App::new(button);
    app.run().unwrap()
}
