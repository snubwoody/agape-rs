use super::Widget;
use crate::element::{Element, ElementKind, LayoutKind};
use crate::impl_style;
use crate::style::BoxStyle;
use agape_core::GlobalId;
use agape_layout::{BlockLayout, Layout};
use agape_renderer::Renderer;
use agape_renderer::rect::Rect;

/// A widget that wraps another widget.
#[derive(Clone)]
pub struct Container<W> {
    id: GlobalId,
    pub child: W,
    pub style: BoxStyle,
    pub padding: u32,
}

impl<W> Container<W> {
    pub fn new(child: W) -> Self {
        Self {
            id: GlobalId::new(),
            style: BoxStyle::new(),
            child,
            padding: 0,
        }
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    impl_style!();
}

impl<W: Widget> Widget for Container<W> {
    fn id(&self) -> GlobalId {
        self.id
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
            label: String::from("Container"),
            children: vec![element],
            on_click: None,
        }
    }

    fn traverse(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
        self.child.traverse(f);
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![&self.child]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;
    use agape_core::{Color, Size};
    use agape_layout::solve_layout;
}
