#![allow(non_snake_case)]
use agape::state::{Context, StateCell};
use agape::widgets::*;
use agape::{App, hstack, vstack};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(TodoList::default()).run()
}

#[derive(Default)]
struct TodoList {
    items: StateCell<Vec<String>>,
}

impl View for TodoList {
    type Widget = VStack;

    fn view(&self, _: &mut Context) -> Self::Widget {
        // TODO: get or init
        let items = self.items.clone();
        let mut item_list = VStack::new().spacing(20);
        for item in items.get().iter() {
            item_list.append_child(Text::new(item));
        }
        let items2 = items.clone();
        vstack![
            hstack![
                Button::text("Add item").on_click(move |_| {
                    items2.update(|items| items.push(String::from("New item")))
                }),
                Button::text("Clear").on_click(move |_| items.update(|items| items.clear())),
            ]
            .spacing(12),
            item_list,
        ]
        .fill()
        .align_center()
        .spacing(12)
    }
}
