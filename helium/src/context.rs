use std::collections::HashMap;
use helium_core::{GlobalId, Position};
use crate::widgets::{Widget, WidgetState};

/// Global app context which keeps track of important
/// information such as the current mouse position and
/// the state of each widget.
/// 
/// The context contains a couple of things:
/// - The mouse position
/// - The gesture state of each widget
#[derive(Debug)]
pub struct Context{
    mouse_position: Position,
    /// Keeps track of the state of all widgets in the 
    /// widget tree.
    state: HashMap<GlobalId,WidgetState>,
}

impl Context{
    pub fn new(widget: &impl Widget) -> Self{
        let mut state = HashMap::new();
        for w in widget.iter(){
            state.insert(w.id(),WidgetState::Resting);
        }
        
        Self{
            mouse_position: Position::default(),
            state
        }
    }
    
    pub fn update_mouse_pos(&mut self, mouse_position: Position){
        self.mouse_position = mouse_position;
    }
    
    pub fn mouse_pos(&self) -> Position{
        self.mouse_position
    }
}

#[cfg(test)]
mod test{
    use crate::{hstack, vstack};
    use crate::widgets::Rect;
    use super::*;
    
    #[test]
    fn init_context(){
        let widget = hstack! {
            Rect::new(100.0,100.0),
            Rect::new(100.0,100.0),
        };
        
        let root_id = widget.id();
        let id1 = widget.children()[0].id();
        let id2 = widget.children()[1].id();
        
        let ctx = Context::new(&widget);
        
        assert!(ctx.state.get(&root_id).is_some());
        assert!(ctx.state.get(&id1).is_some());
        assert!(ctx.state.get(&id2).is_some());
    }
    
    #[test]
    fn state_has_nested_children(){
        let widget = hstack! {
            Rect::new(100.0,100.0),
            vstack! {
                Rect::new(100.0,100.0),
            }
        };
        
        let vstack = widget.children()[1];
        let rect_id = vstack.children()[0].id();
        
        let ctx = Context::new(&widget);
        assert!(ctx.state.get(&rect_id).is_some());
    }
}

