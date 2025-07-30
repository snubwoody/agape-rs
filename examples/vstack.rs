use agape::widgets::{Image, Text};
use agape::{App, vstack};

fn main() -> Result<(), agape::Error> {
    let image = Image::open("./examples/assets/PARTYNEXTDOOR Album Cover.jpg")?;

    let vstack = vstack! {
        image,
        Text::new("PARTYNEXTDOOR").font_size(24),
        Text::new("2013").font_size(12),
    }
    .spacing(12)
    .padding(12)
    .fill()
    .align_center();

    let app = App::new(vstack);
    app.run()
}
