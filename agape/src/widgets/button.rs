use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{LayoutDescription, LayoutType, RenderBox, RenderObject, Text, Widget};
use agape_core::{GlobalId, Position, Size};

pub struct Button {
    id: GlobalId,
    child: Box<dyn Widget>,
    click_fn: Option<Box<dyn FnMut()>>,
    hover_fn: Option<Box<dyn FnMut()>>,
    style: BoxStyle,
}

impl Default for Button {
    fn default() -> Button {
        Button {
            id: GlobalId::new(),
            child: Box::new(Text::new("")),
            click_fn: None,
            hover_fn: None,
            style: BoxStyle::new(),
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

    fn build(&self) -> RenderBox {
        let layout_desc = LayoutDescription {
            layout_type: LayoutType::BlockLayout,
            ..Default::default()
        };

        let render_object = RenderObject::Rect {
            color: self.style.background_color.clone(),
            border: self.style.border.clone(),
        };

        RenderBox {
            id: self.id,
            layout_desc,
            position: Position::default(),
            size: Size::default(),
            render_object,
            children: vec![self.child.build()],
        }
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&*self.child]
    }

    fn children_mut(&mut self) -> &mut [Box<dyn Widget>] {
        std::slice::from_mut(&mut self.child)
    }

    fn traverse(&self, f: &mut dyn FnMut(&dyn Widget)) {
        f(self.child.as_ref());
        self.child.traverse(f);
    }

    fn traverse_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(self.child.as_mut());
        self.child.traverse_mut(f);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::renderer::init_font;
    use crate::{FONT, hstack};

    #[test]
    fn traverse_child() {
        let hstack = hstack! {};
        let id = hstack.id();
        let mut button = Button::new(hstack);

        button.traverse(&mut |widget: &dyn Widget| assert_eq!(id, widget.id()));
        button.traverse_mut(&mut |widget: &mut dyn Widget| assert_eq!(id, widget.id()));
    }

    #[test]
    fn expose_children() {
        FONT.set(init_font()).unwrap();
        let text = Text::new("Hello");
        let id = text.id();

        let button = Button::new(text);
        assert_eq!(button.children()[0].id(), id);
    }
}
