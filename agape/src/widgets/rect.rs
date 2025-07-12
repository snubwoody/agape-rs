use super::Widget;
use crate::style::{Border, BoxStyle};
use crate::view::{RectView, View};
use crate::{Color, impl_style};
use agape_core::{GlobalId, IntoColor, Rgba};
use agape_layout::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Rect {
    id: GlobalId,
    intrinsic_size: IntrinsicSize,
    style: BoxStyle,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        let intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        };

        Self {
            intrinsic_size,
            ..Default::default()
        }
    }

    impl_style!();
}

impl Widget for Rect {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn layout(&self) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = self.intrinsic_size;
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

    #[test]
    fn correct_ids() {
        let rect = Rect::new(100.0, 100.0);
        let layout = rect.layout();
        let view = rect.view();

        assert_eq!(rect.id, layout.id());
        assert_eq!(rect.id, view.id());
    }

    #[test]
    fn view_has_correct_color() {
        let color = Color::rgba(24, 145, 110, 100);
        let rect = Rect::new(100.0, 100.0).background_color(color.clone());
        let view = rect.view();
        assert_eq!(view.color(), &color);
    }
}
