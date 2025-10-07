#![allow(non_snake_case)]
mod messages;
use agape::element::Element;
use agape::widgets::{Button, TextField, VStack, Widget};
use agape::{App, GlobalId, MessageQueue, hstack, vstack, widgets::Text};
use messages::*;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(TodoList::new()).run()
}

struct TodoList {
    id: GlobalId,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            id: GlobalId::default(),
        }
    }
}

#[derive(Default)]
struct Items {
    id: GlobalId,
}

impl Items {
    pub fn new() -> Self {
        Self {
            id: GlobalId::new(),
        }
    }

    pub fn update(&mut self, messages: &mut MessageQueue) {
        if let Some(add_todo) = messages.get::<AddTodo>() {
            // self.widget.append_child(Text::new(&add_todo.0))
        }

        if messages.has::<ClearTodos>() {
            // self.widget.clear();
        }
    }
}

impl Widget for Items {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

    fn build(&self) -> Element {
        vstack![
            Text::new("Item 1"),
            Text::new("Item 2"),
            Text::new("Item 3"),
            Text::new("Item 4"),
        ]
        .spacing(12)
        .build()
    }
}

impl Widget for TodoList {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

    fn build(&self) -> Element {
        vstack![Menu::new(), Items::new(),]
            .spacing(24)
            .padding(16)
            .fill()
            .align_center()
            .build()
    }
}

struct Menu {
    id: GlobalId,
    menu_active: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            id: GlobalId::new(),
            menu_active: false,
        }
    }

    fn update(&mut self, messages: &mut MessageQueue) {
        if messages.has::<EnableMenu>() && !self.menu_active {
            self.menu_active = true;
            // self.widget.append_child(TodoInput::new());
        }

        if messages.has::<DisableMenu>() && self.menu_active {
            self.menu_active = false;
            // self.widget.pop();
        }
    }
}

impl Widget for Menu {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

    fn build(&self) -> Element {
        vstack![MenuBar()].spacing(12).build()
    }
}

fn MenuBar() -> impl Widget {
    hstack![
        Button::new(Text::new("Add item")).on_click(|messages| {
            messages.add(EnableMenu);
            println!("Clicked")
        }),
        Button::new(Text::new("Clear")).on_click(|messages| messages.add(ClearTodos)),
    ]
    .spacing(12)
}

struct TodoInput {
    id: GlobalId,
    current_todo: String,
}

impl TodoInput {
    pub fn new() -> Self {
        Self {
            id: GlobalId::new(),
            current_todo: String::new(),
        }
    }

    pub fn update(&mut self, messages: &mut MessageQueue) {
        if let Some(input) = messages.get::<InputTodo>() {
            self.current_todo = input.0.clone();
        }

        if messages.has::<InsertTodo>() && !self.current_todo.is_empty() {
            messages.add(AddTodo(self.current_todo.clone()));
            messages.add(DisableMenu);
            self.current_todo.clear();
        }
    }
}

impl Widget for TodoInput {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}

    fn build(&self) -> Element {
        vstack![
            TextField::new().on_change(|value, messages| messages.add(InputTodo(value.to_owned()))),
            Button::new(Text::new("Add item")).on_click(|messages| messages.add(InsertTodo)),
        ]
        .spacing(12)
        .build()
    }
}
