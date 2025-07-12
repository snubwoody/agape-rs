use super::Widget;
use crate::impl_style;
use crate::style::BoxStyle;
use crate::view::{RectView, View};
use agape_core::GlobalId;
use agape_layout::{EmptyLayout, IntrinsicSize, Layout};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Rect {
    id: GlobalId,
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

    fn layout(&self) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = self.style.intrinsic_size;
        layout.id = self.id;

        Box::new(layout)
    }

    fn view(&self) -> Box<dyn View> {
        let view = RectView {
            id: self.id,
            color: self.style.background_color.clone(),
            border: self.style.border.clone(),
            ..Default::default()
        };

        Box::new(view)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Color;

    #[test]
    fn layout_attributes() {
        let rect = Rect::new().fill();
        let layout = rect.layout();
        assert_eq!(layout.intrinsic_size(), IntrinsicSize::fill());
    }

    #[test]
    fn correct_ids() {
        let rect = Rect::new();
        let layout = rect.layout();
        let view = rect.view();

        assert_eq!(rect.id, layout.id());
        assert_eq!(rect.id, view.id());
    }

    #[test]
    fn view_has_correct_color() {
        let color = Color::rgba(24, 145, 110, 100);
        let rect = Rect::new().background_color(color.clone());
        let view = rect.view();
        assert_eq!(view.color(), &color);
    }
}
