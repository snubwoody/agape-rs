#![allow(non_snake_case)]
use agape::state::{Context, StateCell};
use agape::widgets::*;
use agape::{App, MessageQueue, hstack, vstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(TodoList::default()).run()
}

#[derive(Debug)]
struct AddTodo(&'static str);

#[derive(Default)]
struct TodoList {
    items: Vec<String>,
}

impl View for TodoList {
    type Widget = VStack;

    fn update(&mut self, msg: &mut MessageQueue) {
        if let Some(message) = msg.get::<AddTodo>() {
            self.items.push(message.0.to_owned())
        }
    }

    fn view(&self, _: &mut Context) -> Self::Widget {
        let items: Vec<Text> = self.items.iter().map(Text::from).collect();

        let item_list = vstack![].with_children(items);

        vstack![
            Text::new("Todo List"),
            Button::text("Add item").on_click(|msg| msg.add(AddTodo("TODO"))),
            item_list,
        ]
        .spacing(12)
        .fill()
        .align_center()
    }
}
