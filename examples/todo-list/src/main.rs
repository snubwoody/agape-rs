#![allow(non_snake_case)]
use agape::state::Context;
use agape::widgets::*;
use agape::{App, MessageQueue, vstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(TodoList::default()).run()
}

#[derive(Debug)]
struct AddTodo;
#[derive(Debug)]
struct InputChange(String);

#[derive(Default)]
struct TodoList {
    items: Vec<String>,
    value: String,
}

impl View for TodoList {
    type Widget = VStack;

    fn update(&mut self, msg: &mut MessageQueue) {
        if msg.has::<AddTodo>() {
            self.items.push(self.value.clone());
            self.value.clear()
        }

        if let Some(message) = msg.get::<InputChange>() {
            self.value = message.0.to_owned();
        }
    }

    fn view(&self, _: &mut Context) -> Self::Widget {
        let items: Vec<Text> = self.items.iter().map(Text::from).collect();

        // TODO: impl From
        let item_list = vstack![].with_children(items);

        vstack![
            Text::new("Todo List"),
            TextField::new().on_change(|text, msg| msg.add(InputChange(text.to_owned()))),
            Button::text("Add item").on_click(|msg| msg.add(AddTodo)),
            item_list,
        ]
        .spacing(12)
        .fill()
        .align_center()
    }
}
