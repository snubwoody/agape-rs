#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use agape::state::{Context, StateCell};
use agape::widgets::{Button, StatelessWidget, VStack, *};
use agape::{App, vstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Page::default()).run()
}

#[derive(Default)]
struct Page {
    count: StateCell<usize>,
}

impl StatelessWidget for Page {
    type Widget = VStack;

    fn build(&self, _: &mut Context) -> Self::Widget {
        let count = self.count.clone();
        vstack![
            Text::new(&format!("{}", self.count.get())).family("Times New Roman"),
            Button::text("Click me").on_click(move |_| count.set(|count| count + 1)),
        ]
        .spacing(8)
        .fill()
        .align_center()
    }
}
