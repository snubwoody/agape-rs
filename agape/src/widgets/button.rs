use super::{Text, Widget};
use crate::element::{Element, ElementKind, LayoutKind};
use crate::style::BoxStyle;
use crate::{MessageQueue, impl_style};
use agape_core::GlobalId;
use agape_layout::{BlockLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;

type Callback = Box<dyn FnMut(&mut MessageQueue)>;

// TODO: add prefix and suffix icon
/// A widget that reacts to click and hover gestures.
///
/// # Example
/// ```
/// use agape::widgets::Button;
///
/// Button::text("Sign up")
///     .on_hover(|_| println!("About to sign up!"))
///     .on_click(|_| println!("Signed up!!!!"));
/// ```
pub struct Button<W> {
    id: GlobalId,
    child: W,
    style: BoxStyle,
    padding: u32,
    hover_callback: Option<Callback>,
    click_callback: Option<fn(&mut MessageQueue)>,
}

impl Button<Text> {
    pub fn text(text: &str) -> Self {
        Self::new(Text::new(text))
    }
}

impl<W> Button<W> {
    pub fn new(child: W) -> Self {
        Self {
            id: GlobalId::new(),
            style: BoxStyle::new(),
            child,
            padding: 0,
            hover_callback: None,
            click_callback: None,
        }
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn on_hover(mut self, f: impl FnMut(&mut MessageQueue) + Send + Sync + 'static) -> Self {
        self.hover_callback = Some(Box::new(f));
        self
    }

    pub fn on_click(mut self, f: fn(&mut MessageQueue)) -> Self {
        self.click_callback = Some(f);
        self
    }

    impl_style!();
}

impl<W: Widget> Widget for Button<W> {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
        self.child.traverse(f);
    }

    fn build(&self) -> Element {
        let element = self.child.build();
        let kind = ElementKind::Rect {
            style: self.style.clone(),
            layout: LayoutKind::Block,
        };

        Element {
            id: self.id,
            kind,
            label: String::from("Button"),
            children: vec![element],
            on_click: self.click_callback,
        }
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }

    fn click(&mut self, messages: &mut MessageQueue) {
        if let Some(f) = &mut self.click_callback {
            f(messages);
        }
    }

    fn hover(&mut self, messages: &mut MessageQueue) {
        if let Some(f) = &mut self.hover_callback {
            f(messages);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;
    use agape_core::{Color, Size};
    use agape_layout::solve_layout;

    #[test]
    fn traverse() {
        let mut button = Button::new(Rect::default());
        let mut ids = vec![];
        button.traverse(&mut |w| {
            ids.push(w.id());
        });
        assert_eq!(ids.len(), 1);
    }
}
