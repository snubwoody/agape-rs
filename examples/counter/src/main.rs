#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use agape::state::Context;
use agape::widgets::{Button, View, *};
use agape::{App, MessageQueue, hstack};

fn main() -> agape::Result<()> {
    App::new(Counter::default()).run()
}

#[derive(Debug)]
struct Increment;
#[derive(Debug)]
struct Decrement;

#[derive(Default)]
struct Counter {
    count: i32,
}

impl View for Counter {
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
            Button::text("Subtract").on_click(|msg| msg.add(Decrement)),
            Text::new(&format!("{}", self.count)),
            Button::text("Add").on_click(|msg| msg.add(Increment)),
        ]
        .spacing(24)
        .fill()
        .align_center()
    }
}
