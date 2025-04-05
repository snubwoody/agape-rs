use helium_core::{colors::BLACK, Color, IntoColor, Position, Rgba};

#[derive(Clone, PartialEq)]
pub struct Icon {
    pub image: ::image::DynamicImage,
    pub position: Position,
    pub color: Color<Rgba>,
}

impl Icon {
    pub fn new(image: ::image::DynamicImage) -> Self {
        Self {
            image,
            position: Position::default(),
            color: BLACK,
        }
    }

    pub fn color(mut self, color: impl IntoColor<Rgba>) -> Self {
        self.color = color.into_color();
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Position { x, y };
        self
    }
}

impl std::fmt::Debug for Icon{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Icon")
			.field("color", &self.color)
			.field("position", &self.position)
			.field("image", &"ImageBuffer<...>")
			.finish()
	}
}
