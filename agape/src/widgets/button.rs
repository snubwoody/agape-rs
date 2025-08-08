use crate::impl_style;
use crate::style::BoxStyle;
use crate::widgets::{LayoutDescription, LayoutType, RenderBox, RenderObject, Text, Widget};
use agape_core::{GlobalId, Position, Size};
use agape_renderer::Renderer;

/// Wraps another widget and responds to gestures.
///
/// # Example
/// ```
/// use agape::widgets::Button;
///
/// let button = Button::text("Here!")
///     .on_hover(||println!("Hovered"))
///     .on_click(||println!("Clicked"));
/// ```
pub struct Button<W> {
    id: GlobalId,
    child: W,
    click_fn: Option<Box<dyn FnMut()>>,
    hover_fn: Option<Box<dyn FnMut()>>,
    style: BoxStyle,
}

impl Button<Text> {
    pub fn text(text: &str) -> Self {
        Self {
            id: GlobalId::new(),
            child: Text::new(text),
            click_fn: None,
            hover_fn: None,
            style: BoxStyle::default(),
        }
    }
}

impl<W: Widget> Button<W> {
    pub fn new(widget: W) -> Self {
        Self {
            id: GlobalId::new(),
            child: widget,
            click_fn: None,
            hover_fn: None,
            style: BoxStyle::default(),
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

impl<W: Widget> Widget for Button<W> {
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

    fn build(&self, renderer: &mut Renderer) -> RenderBox {
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
            children: vec![self.child.build(renderer)],
        }
    }

    fn traverse(&self, f: &mut dyn FnMut(&dyn Widget)) {
        f(&self.child);
        self.child.traverse(f);
    }

    fn traverse_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
        self.child.traverse_mut(f);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hstack;

    #[test]
    fn traverse_child() {
        let hstack = hstack! {};
        let id = hstack.id();
        let mut button = Button::new(hstack);

        button.traverse(&mut |widget: &dyn Widget| assert_eq!(id, widget.id()));
        button.traverse_mut(&mut |widget: &mut dyn Widget| assert_eq!(id, widget.id()));
    }
}
