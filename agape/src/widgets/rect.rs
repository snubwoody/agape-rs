use super::{LayoutDescription, LayoutType, RenderBox, RenderObject, Widget};
use crate::impl_style;
use crate::style::BoxStyle;
use agape_core::{GlobalId, Position, Size};
use agape_renderer::Renderer;

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

    fn build(&self, _: &mut Renderer) -> RenderBox {
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
}
