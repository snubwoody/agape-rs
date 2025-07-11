use agape_core::{Color, IntoColor, Rgba};
use agape_layout::{BoxSizing, IntrinsicSize};

#[derive(Default, Debug, PartialOrd, PartialEq, Clone)]
pub struct BoxStyle {
    pub intrinsic_size: IntrinsicSize,
    pub background_color: Color<Rgba>,
    pub border: Option<Border>,
}

impl BoxStyle {
    /// Create a new [`BoxStyle`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the intrinsic width.
    ///
    /// # Example
    /// ```
    /// use agape::{style::BoxStyle,layout::BoxSizing};
    ///
    /// let mut style = BoxStyle::new();
    /// style.intrinsic_width(BoxSizing::Flex(8));
    ///
    /// assert_eq!(style.intrinsic_size.width,BoxSizing::Flex(8));
    /// ```
    pub fn intrinsic_width(&mut self, width: BoxSizing) {
        self.intrinsic_size.width = width;
    }

    /// Set the intrinsic height.
    ///
    /// # Example
    /// ```
    /// use agape::{style::BoxStyle,layout::BoxSizing};
    ///
    /// let mut style =  BoxStyle::new();
    /// style.intrinsic_height(BoxSizing::Fixed(140.0));
    ///
    /// assert_eq!(style.intrinsic_size.height,BoxSizing::Fixed(140.0));
    /// ```
    pub fn intrinsic_height(&mut self, height: BoxSizing) {
        self.intrinsic_size.height = height;
    }

    /// Set the intrinsic width and height to `BoxSizing::Fill`.
    ///
    /// # Example
    /// ```
    /// use agape::{style::BoxStyle,layout::IntrinsicSize};
    ///
    /// let mut style = BoxStyle::new();
    /// style.fill();
    ///
    /// assert_eq!(style.intrinsic_size,IntrinsicSize::fill());
    /// ```
    pub fn fill(&mut self) {
        self.intrinsic_width(BoxSizing::Flex(1));
        self.intrinsic_height(BoxSizing::Flex(1));
    }

    /// Set the border width.
    pub fn border_width(&mut self, width: f32) {
        match &mut self.border {
            Some(border) => {
                border.width = width;
            }
            None => {
                let border = Border {
                    width,
                    ..Default::default()
                };
                self.border = Some(border);
            }
        }
    }

    /// Set the border color.
    pub fn border_color(&mut self, color: impl IntoColor<Rgba>) {
        match &mut self.border {
            Some(border) => {
                border.color = color.into_color();
            }
            None => {
                let border = Border {
                    color: color.into_color(),
                    ..Default::default()
                };
                self.border = Some(border);
            }
        }
    }
}

/// Border style for [`View`]s;
#[derive(Clone, PartialEq, Debug, Default, PartialOrd)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_border_width() {
        let mut style = BoxStyle::new();
        style.border_width(12.0);

        assert_eq!(style.border.unwrap().width, 12.0);
    }
}
