use crystal::{BlockLayout, Layout};
use helium_core::{Color, GlobalId, Position, Rgba};
use crate::context::Context;
use crate::view::{RectView, View};
use crate::widgets::{Text, Widget};

pub struct Button{
    id: GlobalId,
    color: Color<Rgba>,
    child: Box<dyn Widget>,
    padding: u32,
    click_fn: Option<Box<dyn FnMut()>>,
    mouse_pos: Position,
}

impl Default for Button{
    fn default() -> Button{
        Button{
            id: GlobalId::new(),
            color: Color::TRANSPARENT,
            padding: 0,
            child: Box::new(Text::new("")),
            click_fn: None,
            mouse_pos: Position::default(),
        }
    }
}

impl Button{
    pub fn new(widget: impl Widget + 'static) -> Self{
        Self{
            child: Box::new(widget),
            ..Self::default()
        }
    }
    
    /// Check if the mouse position is over the button
    fn is_hovered(&self) -> bool{
        self.layout().bounds().within(&self.mouse_pos)
    }
    
    pub fn on_click(mut self, callback: impl FnMut() + 'static) -> Self{
        self.click_fn = Some(Box::new(callback));
        self
    }
}

impl Widget for Button{
    fn id(&self) -> GlobalId {
        self.id
    }

    fn tick(&mut self, cx: &Context) {
        dbg!(cx);
    }
    
    fn view(&self) -> Box<dyn View> {
        let mut view = RectView::new(self.color.clone());
        view.set_id(self.id);
        Box::new(view)
    }

    fn layout(&self) -> Box<dyn Layout> {
        let child = self.child.layout();
        let mut layout = BlockLayout::new(child);
        layout.id = self.id;
        layout.padding = self.padding;
        Box::new(layout)
    }

    fn handle_click(&mut self) {
        if let Some(func) = &mut self.click_fn{  
            func();
        }
    }

    fn handle_cursor(&mut self, position: Position) {
        dbg!(position);
    }
}

#[cfg(test)]
mod test{
    use crate::widgets::Rect;
    use super::*;
    
    #[test]
    fn is_hovered(){
        let mut button = Button::new(Rect::new(100.0,100.0));
        button.mouse_pos = Position::new(24.0,42.4);
        dbg!(button.layout());
        assert!(button.is_hovered());
    }
    
    #[test]
    fn view_and_layout(){
        let button = Button::new(Text::new("Click me"));
        assert_eq!(button.layout().id(), button.id);
        assert_eq!(button.view().id(),button.id)
    }
}