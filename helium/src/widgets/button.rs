use crate::Context;
use crate::view::{RectView, View};
use crate::widgets::{Text, Widget};
use crate::{AppEvent, impl_style};
use crystal::{BlockLayout, Layout};
use helium_core::{Color, GlobalId, Rgba};

pub struct Button {
    id: GlobalId,
    color: Color<Rgba>,
    child: Box<dyn Widget>,
    padding: u32,
    click_fn: Option<Box<dyn FnMut()>>,
    hover_fn: Option<Box<dyn FnMut()>>,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            id: GlobalId::new(),
            color: Color::TRANSPARENT,
            padding: 0,
            child: Box::new(Text::new("")),
            click_fn: None,
            hover_fn: None,
        }
    }
}

impl Button {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            child: Box::new(widget),
            ..Self::default()
        }
    }

    pub fn on_click(mut self, callback: impl FnMut() + 'static) -> Self {
        self.click_fn = Some(Box::new(callback));
        self
    }

    pub fn on_hover(mut self, callback: impl FnMut() + 'static) -> Self {
        self.hover_fn = Some(Box::new(callback));
        self
    }

    impl_style!();
}

impl Widget for Button {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn tick(&mut self, cx: &Context) {
        for event in cx.query_events() {
            match event {
                AppEvent::Hovered(id) => {
                    if *id == self.id() {
                        self.hover()
                    }
                },
                AppEvent::Clicked(id) => {
                    if *id == self.id() {
                        self.click()
                    }
                }
                _ => {}
            }
        }
    }

    fn click(&mut self) {
        if let Some(func) = &mut self.click_fn {
            func();
        }
    }

    fn hover(&mut self) {
        if let Some(func) = &mut self.hover_fn {
            func();
        }
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

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&*self.child]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expose_children() {
        let text = Text::new("Hello");
        let id = text.id();

        let button = Button::new(text);
        assert_eq!(button.children()[0].id(), id);
    }

    #[test]
    fn view_and_layout() {
        let button = Button::new(Text::new("Click me"));
        assert_eq!(button.layout().id(), button.id);
        assert_eq!(button.view().id(), button.id)
    }
}
