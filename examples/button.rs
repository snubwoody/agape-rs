use helium::App;
use helium::widgets::{Button, Text};

fn main(){
    let button = Button::new(Text::new("Click me"))
        .on_click(||println!("I have been clicked!"));

    let app = App::new(button);
    app.run().unwrap()
}