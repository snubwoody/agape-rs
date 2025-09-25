use agape::widgets::{Button, HStack, VStack};
use agape::{App, GlobalId, MessageQueue, Widget, hstack, vstack, widgets::Text};

fn main() -> agape::Result<()> {
    tracing_subscriber::fmt::init();
    App::new(TodoList::new()).run()
}

#[derive(Clone, Copy)]
struct AddTodo(&'static str);
#[derive(Clone, Copy)]
struct ClearTodos;

#[derive(Widget)]
struct TodoList {
    id: GlobalId,
    #[child]
    child: VStack,
}

impl TodoList {
    pub fn new() -> Self {
        let child = vstack![Menu::new(), Items::new(),].spacing(24).padding(16);
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
            self.widget.append_child(Text::new(add_todo.0))
        }

        if messages.has::<ClearTodos>() {
            self.widget.clear();
        }
    }
}

#[derive(Widget)]
struct Menu {
    id: GlobalId,
    #[child]
    widget: HStack,
}

impl Menu {
    pub fn new() -> Self {
        let widget = hstack![
            Button::new(Text::new("Add item")).on_click(|messages| messages.add(AddTodo("Item"))),
            Button::new(Text::new("Clear")).on_click(|messages| messages.add(ClearTodos)),
        ]
        .spacing(12);

        Self {
            id: GlobalId::new(),
            widget,
        }
    }
}
