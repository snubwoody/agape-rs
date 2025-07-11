use agape_core::{Color, Rgba};

/// Border style for [`View`]s;
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Border {
    pub width: f32,
    pub color: Color<Rgba>,
}

impl Border {
    /// Create a new border.
    pub fn new() -> Self {
        Self::default()
    }
}
