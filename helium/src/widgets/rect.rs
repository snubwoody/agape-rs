use super::Widget;
use crate::Color;
use crystal::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use helium_core::{GlobalId, IntoColor, Rgba, colors::WHITE};
use crate::view::{RectView, View};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rect {
    id: GlobalId,
    intrinsic_size: crystal::IntrinsicSize,
    color: Color<Rgba>,
    corner_radius: u32,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        let intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        };

        Self {
            id: GlobalId::default(),
            color: WHITE,
            intrinsic_size,
            corner_radius: 0,
        }
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
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
        let mut view = RectView::new(self.color.clone());
        view.set_id(self.id);
        Box::new(view)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn correct_ids(){
        let rect = Rect::new(100.0, 100.0);
        let layout = rect.layout();
        let view = rect.view();
        
        assert_eq!(rect.id,layout.id());
        assert_eq!(rect.id,view.id());
    }
    
    #[test]
    fn view_has_correct_color(){
        let color = Color::rgba(24,145,110,100);
        let rect = Rect::new(100.0, 100.0).color(color.clone());
        let view = rect.view();
        assert_eq!(view.color(),&color);
    }
}

