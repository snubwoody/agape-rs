use agape::widgets::*;
use agape::{App, hstack, vstack};

fn main() -> Result<(), agape::Error> {
    let image = Image::open("./examples/assets/PARTYNEXTDOOR Album Cover.jpg")?.fixed(250.0, 250.0);
    let svg = Svg::open("agape/icons/feather-icons/calendar.svg")?;

    let vstack = vstack! {
        image,
        Text::new("PARTYNEXTDOOR").font_size(24),
        hstack!{
            svg,
            Text::new("2013").font_size(12)
        }
        .spacing(8)
        .align_center(),
    }
    .spacing(12)
    .padding(12)
    .fill()
    .align_center();

    let app = App::new(vstack);
    app.run()
}
