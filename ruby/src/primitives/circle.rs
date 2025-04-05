use helium_core::{Color, IntoColor, Position, Rgba};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Circle {
    pub diameter: f32,
    pub position: Position,
    pub color: Color<Rgba>,
}

impl Circle {
    pub fn new(diameter: f32) -> Self {
        Self {
            diameter,
            position: Position::default(),
            color: Color::default(),
        }
    }

    pub fn unit(value: f32) -> Self {
        Self {
            diameter: value,
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
}
