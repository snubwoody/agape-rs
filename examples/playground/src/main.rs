use agape::state::StateCell;
use agape::widgets::{Button, HStack, StatelessWidget, TextField, VStack, *};
use agape::{App, GlobalId, Widget, hstack, vstack};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

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

    fn build(&self) -> Self::Widget {
        let count = self.count.clone();
        vstack![
            Text::new(&format!("{}", self.count.get())),
            Button::text("Click me").on_click(move |_| count.set(|count| count + 1)),
        ]
        .spacing(8)
        .fill()
        .align_center()
    }
}
