#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use agape::state::Context;
use agape::widgets::{Button, View, *};
use agape::{App, MessageQueue, hstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(Page::default()).run()
}

#[derive(Debug)]
struct Increment;
#[derive(Debug)]
struct Decrement;

#[derive(Default)]
struct Page {
    count: i32,
}

impl View for Page {
    type Widget = HStack;

    fn update(&mut self, msg: &mut MessageQueue) {
        if msg.has::<Increment>() {
            self.count += 1;
        }

        if msg.has::<Decrement>() {
            self.count -= 1;
        }
    }

    fn view(&self, _: &mut Context) -> Self::Widget {
        hstack![
            Button::text("Decrement").on_click(|msg| msg.add(Decrement)),
            Text::new(&format!("{}", self.count)),
            Button::text("Increment").on_click(|msg| msg.add(Increment)),
        ]
        .spacing(8)
        .fill()
        .align_center()
    }
}
