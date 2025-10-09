#![allow(non_snake_case)]
use agape::widgets::*;
use agape::{App, hstack, vstack};
use std::sync::{Arc, Mutex};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(TodoList::default()).run()
}

#[derive(Debug, Default)]
struct TodoList {
    items: Arc<Mutex<Vec<String>>>,
}

impl StatelessWidget for TodoList {
    type Widget = VStack;

    fn build(&self) -> Self::Widget {
        // TODO: get or init
        let mut item_list = VStack::new().spacing(20);
        let todo_items = self.items.clone();
        let todo_items2 = self.items.clone();
        // let mut items = todo_items.lock().unwrap();
        for item in self.items.lock().unwrap().iter() {
            item_list.append_child(Text::new(item));
        }
        vstack![
            hstack![
                Button::text("Add item")
                    .on_click(move |_| todo_items.lock().unwrap().push(String::from("New item"))),
                Button::text("Clear").on_click(move |_| todo_items2.lock().unwrap().clear()),
            ]
            .spacing(12),
            item_list,
            // vstack![Text::new("Wash clothes"), Text::new("Take out the trash"),].spacing(20)
        ]
        .fill()
        .align_center()
        .spacing(12)
    }
}
