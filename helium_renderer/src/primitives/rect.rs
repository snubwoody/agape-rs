use super::{IntoSurface, Surface};
use helium_core::{Color, IntoColor, Position, Rgba, Size};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct RectSurface {
    pub size: Size,
    pub position: Position,
    pub color: Color<Rgba>,
    pub corner_radius: f32,
}

impl RectSurface {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Size { width, height },
            ..Default::default()
        }
    }

    pub fn unit(value: f32) -> Self {
        Self {
            size: Size::unit(value),
            ..Default::default()
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    pub fn corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }
}

impl IntoSurface for RectSurface {
    fn into_surface(self) -> Surface {
        Surface::Rect(self)
    }
}

impl From<&dyn crystal::Layout> for RectSurface {
    fn from(layout: &dyn crystal::Layout) -> Self {
        let size = layout.size();
        let position = layout.position();

        Self::new(size.width, size.height).position(position.x, position.y)
    }
}
