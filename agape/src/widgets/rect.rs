use super::Widget;
use crate::Color;
use crate::style::Border;
use crate::view::{RectView, View};
use agape_core::{GlobalId, IntoColor, Rgba};
use agape_layout::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Rect {
    id: GlobalId,
    intrinsic_size: IntrinsicSize,
    color: Color<Rgba>,
    border: Option<Border>,
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

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    /// Set the intrinsic width to `BoxSixing::Flex`.
    pub fn flex_width(mut self, factor: u8) -> Self {
        self.intrinsic_size.width = BoxSizing::Flex(factor);
        self
    }

    /// Set the intrinsic height to `BoxSixing::Flex`.
    pub fn flex_height(mut self, factor: u8) -> Self {
        self.intrinsic_size.height = BoxSizing::Flex(factor);
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        match &mut self.border {
            Some(border) => {
                border.width = width;
            }
            None => {
                self.border = Some(Border {
                    width,
                    ..Default::default()
                });
            }
        }

        self
    }
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
            color: self.color.clone(),
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
        let rect = Rect::new(100.0, 100.0).color(color.clone());
        let view = rect.view();
        assert_eq!(view.color(), &color);
    }
}
