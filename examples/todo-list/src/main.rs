#![allow(non_snake_case)]
use agape::widgets::{Button, HStack, TextField, VStack};
use agape::{App, GlobalId, MessageQueue, Widget, hstack, vstack, widgets::Text};
use log::info;

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(TodoList::new()).run()
}

#[derive(Clone, Debug)]
struct AddTodo(String);
#[derive(Clone, Copy, Debug, PartialEq)]
struct InsertTodo;
#[derive(Clone, Debug, Copy, PartialEq)]
struct EnableMenu;
#[derive(Clone, Copy, PartialEq, Debug)]
struct DisableMenu;
#[derive(Clone, Copy, Debug)]
struct ClearTodos;
#[derive(Clone, PartialEq, Debug)]
struct InputTodo(String);

#[derive(Widget)]
struct TodoList {
    id: GlobalId,
    #[child]
    child: VStack,
}

impl TodoList {
    pub fn new() -> Self {
        let child = vstack![Menu::new(), Items::new(),]
            .spacing(24)
            .padding(16)
            .fill()
            .align_center();

        Self {
            id: GlobalId::default(),
            child,
        }
    }
}

#[derive(Default, Widget)]
#[interactive]
struct Items {
    id: GlobalId,
    #[child]
    widget: VStack,
}

impl Items {
    pub fn new() -> Self {
        // TODO add x button to clear
        let widget = vstack![
            Text::new("Item 1"),
            Text::new("Item 2"),
            Text::new("Item 3"),
            Text::new("Item 4"),
        ]
        .spacing(12);

        Self {
            id: GlobalId::new(),
            widget,
        }
    }

    pub fn update(&mut self, messages: &mut MessageQueue) {
        if let Some(add_todo) = messages.get::<AddTodo>() {
            self.widget.append_child(Text::new(&*add_todo.0))
        }

        if messages.has::<ClearTodos>() {
            self.widget.clear();
        }
    }
}

#[derive(Widget)]
#[interactive]
struct Menu {
    id: GlobalId,
    #[child]
    widget: VStack,
    menu_active: bool,
}

impl Menu {
    pub fn new() -> Self {
        let widget = vstack![MenuBar()].spacing(12);

        Self {
            id: GlobalId::new(),
            widget,
            menu_active: false,
        }
    }

    fn update(&mut self, messages: &mut MessageQueue) {
        if messages.has::<EnableMenu>() && !self.menu_active {
            self.menu_active = true;
            self.widget.append_child(TodoInput::new());
        }

        if messages.has::<DisableMenu>() && self.menu_active {
            self.menu_active = false;
            self.widget.pop();
        }
    }
}

fn MenuBar() -> impl agape::widgets::Widget {
    hstack![
        Button::new(Text::new("Add item")).on_click(|messages| messages.add(EnableMenu)),
        Button::new(Text::new("Clear")).on_click(|messages| messages.add(ClearTodos)),
    ]
    .spacing(12)
}

#[derive(Widget)]
#[interactive]
struct TodoInput {
    id: GlobalId,
    #[child]
    child: VStack,
    current_todo: String,
}

impl TodoInput {
    pub fn new() -> Self {
        let child = vstack![
            TextField::new().on_change(|value, messages| messages.add(InputTodo(value.to_owned()))),
            Button::new(Text::new("Add item")).on_click(|messages| messages.add(InsertTodo)),
        ]
        .spacing(12);

        Self {
            id: GlobalId::new(),
            child,
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
