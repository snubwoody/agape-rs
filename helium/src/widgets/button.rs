use crystal::Layout;
use helium_core::{Color, GlobalId, Rgba};
use crate::view::View;
use crate::widgets::{Text, Widget};

pub struct Button{
    id: GlobalId,
    color: Color<Rgba>,
    child: Box<dyn Widget>,
    click_fn: Option<Box<dyn FnMut()>>
}

impl Default for Button{
    fn default() -> Button{
        Button{
            id: GlobalId::new(),
            color: Color::TRANSPARENT,
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
}

impl Widget for Button{
    fn id(&self) -> GlobalId {
        todo!()
    }
    
    fn view(&self) -> Box<dyn View> {
        todo!()
    }

    fn layout(&self) -> Box<dyn Layout> {
        todo!()
    }
}