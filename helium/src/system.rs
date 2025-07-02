use std::any::{Any, TypeId};
use std::collections::HashMap;
use crystal::Layout;
use helium_core::Position;
use crate::AppEvent;
use crate::view::View;

// Things I need to access:
// - Layout
// - Widgets?
// - Views
// - Event queue
// - Mouse position

struct Resources{
    events: Vec<AppEvent>,
    mouse_position: Position,
    views: Vec<Box<dyn View>>,
    layout: Box<dyn Layout>,
}

fn render_system(layout:&dyn Layout, views: Vec<Box<dyn View>>) {
    
}

fn layout_system(layout: &mut Box<dyn Layout>){
    
}

fn animation_system(layout: &mut dyn Layout, views: &[&mut dyn View]){
    
}

fn event_system(events: &Vec<AppEvent>,layout: &dyn Layout)  {
    
}

trait System<Input> {
    fn run(&mut self, resources: &mut Resources);
}

impl <F,T1,L> System<(T1,L)> for F
where 
    F: FnMut(T1,L),
    T1:'static,
    L: Layout
{
    fn run(&mut self,resources: &mut Resources){
        
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_system(){
        // let mut k = |query: Query<&dyn Widget>|{};
    }
}