use agape::App;
use agape::widgets::Text;

fn main() -> Result<(), agape::Error> {
    let text = Text::new("Hello, world!");

    let app = App::new(text);
    app.run()
}
