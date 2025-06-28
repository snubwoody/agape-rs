use std::collections::HashMap;
use helium_core::{GlobalId, Position};
use crate::widgets::WidgetState;

/// Global app context which keeps track of important
/// information such as the current mouse position and
/// the state of each widget.
#[derive(Debug)]
pub struct Context{
    mouse_position: Position,
    /// Keeps track of the state of all widgets in the 
    /// widget tree
    state: HashMap<GlobalId,WidgetState>,
}

impl Context{
    pub fn new() -> Self{
        Self{
            mouse_position: Position::default(),
            state: HashMap::new(),
        }
    }
    
    pub fn update_mouse_pos(&mut self, mouse_position: Position){
        self.mouse_position = mouse_position;
    }
    
    pub fn mouse_pos(&self) -> Position{
        self.mouse_position
    }
}

