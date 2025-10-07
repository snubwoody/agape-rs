use super::Widget;
use crate::element::{Element, ElementKind, LayoutKind};
use crate::impl_style;
use crate::style::BoxStyle;
use agape_core::{GlobalId, Position, Size};
use agape_layout::{EmptyLayout, Layout};
use agape_renderer::Renderer;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Rect {
    id: GlobalId,
    size: Size,
    position: Position,
    style: BoxStyle,
}

impl Rect {
    pub fn new() -> Self {
        Self::default()
    }

    impl_style!();
}

impl Widget for Rect {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn children(&self) -> Vec<&dyn Widget> {
        vec![]
    }

    fn build(&self) -> Element {
        Element {
            id: GlobalId::new(),
            kind: ElementKind::Rect {
                layout: LayoutKind::Empty,
                style: self.style.clone(),
            },
            on_click: None,
            label: String::from("Rect"),
            children: Vec::new(),
        }
    }

    fn traverse(&mut self, _: &mut dyn FnMut(&mut dyn Widget)) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::widgets::Rect;
    use agape_core::{Color, Size};
    use agape_layout::solve_layout;
}
