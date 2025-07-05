//! Systems are functions that are stored in the [`App`] and run every frame.
//! They have a `&mut` to the global [`Context`] allowing it to be modified.
//! 
//! # Create a system
//! ```
//! use agape::{hstack, App, Context};
//! use agape::system::{IntoSystem,System};
//!
//! fn current_mouse_position(cx: &mut Context){
//!     println!("Current mouse position: {}",cx.mouse_pos());
//! }
//!
//! let app = App::new(hstack! {})
//!     .add_system(current_mouse_position);
//! ```

use std::any::{Any, TypeId};
use agape_core::{Position, Size};
use crate::{AppEvent, Context};

// EventReader<E>
// EventWriter<E>
//
//  Only run on the event
//  fn event(cx: Context, event: Event<T>){
//
//  }
//
//

// .add_system(Event<T>,fn)
// .add_system(Update,fn)


/// A [`System`] is a stored procedure that has mutable access
/// to the global [`Context`] object.
pub trait System {
    
    fn run(&mut self, cx: &mut Context,event: AppEvent);

    fn event_type(&self) -> Option<TypeId>{
        None
    }
}

/// A trait for creating systems
pub trait IntoSystem {
    type System: System;

    /// Convert a closure or function into a [`System`].
    fn into_system(self) -> Self::System;
}

trait IntoSystemParts{
    fn into_system_parts(self) -> Self;
}

impl IntoSystemParts for &mut Context{
    fn into_system_parts(self) -> Self {
        self
    }
}

impl IntoSystemParts for AppEvent{
    fn into_system_parts(self) -> Self{
        self
    }
}

impl<F: FnMut(&mut Context)> IntoSystem for F {
    type System = FunctionSystem<Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem { f: self }
    }
}

pub struct FunctionSystem<F> {
    f: F,
}

impl<F: FnMut(&mut Context)> System for FunctionSystem<F> {
    fn run(&mut self, cx: &mut Context,_: AppEvent) {
        (self.f)(cx)
    }
}

/// A system that runs only when a specific event is emitted.
pub struct EventSystem<F>{
    f:F,
}

impl<F> System for EventSystem<F>
where 
    F: FnMut(&mut Context,&AppEvent),
{
    fn run(&mut self, cx: &mut Context,event: AppEvent) {
        (self.f)(cx,&event);
    }
}


#[derive(Debug)]
struct MousePosition(Position);

struct WindowSize(Size);


struct Resources{
    items: Vec<Box<dyn Any>>,
}

impl Resources{
    fn insert<T:'static>(&mut self,item: T){
        // Don't insert the same resource twice
        if let None = self.get::<T>(){
            self.items.push(Box::new(item));
        }
    }
    
    fn get<T:'static>(&self) -> Option<&T>{
        for items in &self.items{
            match items.downcast_ref::<T>(){
                Some(item) => return Some(item),
                None => continue,
            }    
        }
        
        None
    }
}

#[cfg(test)]
mod test{
    use std::time::Instant;
    use agape_core::GlobalId;
    use super::*;
    
    #[test]
    fn get_resource(){
        let instance = Instant::now();
        let mut resources = Resources{items: vec![]};
        resources.items.push(Box::new(MousePosition(Position::default())));
        
        let position = resources.get::<MousePosition>();    
        assert!(position.is_some());        
    }

    #[test]
    fn event_type(){
        let id = AppEvent::Hovered(GlobalId::new()).type_id();
        let id2 = AppEvent::Hovered(GlobalId::new()).type_id();
        dbg!(id,id2);
    }
    
}