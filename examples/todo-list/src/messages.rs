/// Add an item to the todo list.
#[derive(Clone, Debug)]
pub struct AddTodo(pub String);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InsertTodo;
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct EnableMenu;
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DisableMenu;
#[derive(Clone, Copy, Debug)]
pub struct ClearTodos;
#[derive(Clone, PartialEq, Debug)]
pub struct InputTodo(pub String);
