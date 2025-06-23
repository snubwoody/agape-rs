use crystal::{BlockLayout, Layout};
use helium_core::{Color, GlobalId, Rgba};
use crate::view::{RectView, View};
use crate::widgets::{Text, Widget};

pub struct Button{
    id: GlobalId,
    color: Color<Rgba>,
    child: Box<dyn Widget>,
    padding: u32,
    click_fn: Option<Box<dyn FnMut()>>
}

impl Default for Button{
    fn default() -> Button{
        Button{
            id: GlobalId::new(),
            color: Color::TRANSPARENT,
            padding: 0,
            child: Box::new(Text::new("")),
            click_fn: None,
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
    
    pub fn on_click(mut self, callback: impl FnMut() + 'static) -> Self{
        self.click_fn = Some(Box::new(callback));
        self
    }
}

impl Widget for Button{
    fn id(&self) -> GlobalId {
        self.id
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
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn view_and_layout(){
        let button = Button::new(Text::new("Click me"));
        assert_eq!(button.layout().id(), button.id);
        assert_eq!(button.view().id(),button.id)
    }
}