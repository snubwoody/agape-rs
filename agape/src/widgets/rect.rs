use super::{LayoutDescription, LayoutType, RenderBox, RenderObject, Widget};
use crate::impl_style;
use crate::style::BoxStyle;
use crate::view::{RectView, View};
use agape_core::{GlobalId, Position, Size};
use agape_layout::{EmptyLayout, Layout};

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

    fn build(&self) -> RenderBox {
        let layout_desc = LayoutDescription {
            intrinsic_size: self.style.intrinsic_size,
            layout_type: LayoutType::EmptyLayout,
            ..Default::default()
        };

        let render_object = RenderObject::Rect {
            color: self.style.background_color.clone(),
            border: self.style.border.clone(),
        };

        RenderBox {
            id: self.id,
            size: Size::default(),
            position: Position::default(),
            children: Vec::new(),
            layout_desc,
            render_object,
        }
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
    use crate::layout::IntrinsicSize;

    #[test]
    fn view_has_correct_color() {
        let color = Color::rgba(24, 145, 110, 100);
        let rect = Rect::new().background_color(color.clone());
        let view = rect.view();
        assert_eq!(view.color(), &color);
    }
}
