use crate::map;
use std::num::ParseIntError;
use thiserror::Error;

/// Trait for generating [`Color`]'s
pub trait IntoColor<Container> {
    fn into_color(self) -> Color<Container>;
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ColorError {
    #[error("Missing # at the start of hex code")]
    MissingHex,
    #[error("Hex code is {0} characters long but must have a length of 6 or 8")]
    InvalidLength(usize),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

// TODO change this to static string and make color copy
/// Container for hexadecimal colors
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hex(String);

/// Container for rgba colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8, // TODO: make alpha channel an f32?
}

impl Rgba {
    const fn new(r: u8, b: u8, g: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// A color representation. [`Color`] is a generic wrapper around
/// different color formats.
///
/// ## Create an [`Rgba`] color
/// ```
/// use agape_core::Color;
/// let color = Color::rgba(237,102,50,100);
/// ```
///
/// Note that the alpha channel goes from 0-100.
///
/// ## Create a [`Hex`] color.
/// `hex` returns a result.
/// ```
/// use agape_core::Color;
/// let color = Color::hex("#000000");
/// assert!(color.is_ok())
/// ```
/// Use the `hex!` macro for a more convenient way of creating
/// hex colors.
///
/// [`hex`]: Color<Hex>::hex
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color<C = Rgba>(C);

impl Color<Rgba> {
    pub const WHITE: Color<Rgba> = Color::rgb(255, 255, 255);
    pub const BLACK: Color<Rgba> = Color::rgb(0, 0, 0);
    pub const RED: Color<Rgba> = Color::rgb(255, 0, 0);
    pub const GREEN: Color<Rgba> = Color::rgb(0, 255, 0);
    pub const BLUE: Color<Rgba> = Color::rgb(0, 0, 255);
    pub const AMBER: Color<Rgba> = Color::rgb(245, 158, 11);
    pub const TEAL: Color<Rgba> = Color::rgb(128, 225, 214);
    pub const TRANSPARENT: Color<Rgba> = Color::rgba(0, 0, 0, 0);

    /// Create a new rgb [`Colour`].
    ///
    /// # Example
    ///
    /// ```
    /// use agape_core::Color;
    ///
    /// let color = Color::rgb(75,25,90);
    ///
    /// assert_eq!(color.r(),75);
    /// assert_eq!(color.g(),25);
    /// assert_eq!(color.b(),90);
    /// assert_eq!(color.a(),100);
    /// ```
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color<Rgba> {
        let color = Rgba::new(r, b, g, 100);
        Color(color)
    }

    /// Create a new rgba [`Color`]
    ///
    /// # Example
    ///
    /// ```
    /// use agape_core::Color;
    ///
    /// let color = Color::rgba(175,225,90,100);
    ///
    /// assert_eq!(color.r(),175);
    /// assert_eq!(color.g(),225);
    /// assert_eq!(color.b(),90);
    /// assert_eq!(color.a(),100);
    /// ```
    ///
    /// The alpha channel is from 0-100, any values above 100 will
    /// be clipped to 100.
    ///
    /// ```
    /// use agape_core::Color;
    /// let color = Color::rgba(0,0,0,235);
    ///
    /// assert_eq!(color.a(),100);
    /// ```
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color<Rgba> {
        let mut a = a;
        if a > 100 {
            a = 100
        }
        let color = Rgba::new(r, b, g, a);
        Color(color)
    }

    /// Get the red component of the [`Color`]
    pub fn r(&self) -> u8 {
        self.0.r
    }

    /// Get the green component of the [`Color`]
    pub fn g(&self) -> u8 {
        self.0.g
    }

    /// Get the blue component of the [`Color`]
    pub fn b(&self) -> u8 {
        self.0.b
    }

    /// Get the alpha component of the [`Color`]
    pub fn a(&self) -> u8 {
        self.0.a
    }

    /// Get all the inner color components
    pub fn inner(&self) -> (u8, u8, u8, u8) {
        (self.r(), self.g(), self.b(), self.a())
    }

    /// Normalize the colors and convert them from `srgb` to linear `rgb`.
    pub fn normalize(&self) -> [f32; 4] {
        // TODO test the values
        let r = ((self.r() as f32 / 255.0 + 0.055) / 1.055).powf(2.4);
        let g = ((self.g() as f32 / 255.0 + 0.055) / 1.055).powf(2.4);
        let b = ((self.b() as f32 / 255.0 + 0.055) / 1.055).powf(2.4);
        let a = map(self.a() as f32, [0.0, 100.0], [0.0, 1.0]);

        [r, g, b, a]
    }
}

impl Color<Hex> {
    /// Create a new hex [`Color`].
    ///
    /// This method is fallible, as not every string is a valid hex code. Hex codes
    /// must start with `#` and must contain a valid hexadecimal string with 6  or 8
    /// characters.
    ///
    /// For an easier way to create hex colors use the `hex!` macro.
    ///
    /// # Example
    ///
    /// ```
    /// use agape_core::{Color,ColorError};
    ///
    /// fn main() -> Result<(),ColorError>{
    ///     let color = Color::hex("#FFFFFF")?;
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    pub fn hex(value: &str) -> Result<Color<Hex>, ColorError> {
        let hex_code = value.strip_prefix("#").ok_or(ColorError::MissingHex)?;

        match hex_code.len() {
            6 => {
                let (red, green, blue) = (&hex_code[0..2], &hex_code[2..4], &hex_code[4..6]);

                let _ = u8::from_str_radix(red, 16)?;
                let _ = u8::from_str_radix(green, 16)?;
                let _ = u8::from_str_radix(blue, 16)?;

                let mut hex = value.to_string();
                hex.push_str("FF");

                Ok(Color(Hex(hex)))
            }
            8 => {
                let (red, green, blue, alpha) = (
                    &hex_code[0..2],
                    &hex_code[2..4],
                    &hex_code[4..6],
                    &hex_code[6..8],
                );

                let _ = u8::from_str_radix(red, 16)?;
                let _ = u8::from_str_radix(green, 16)?;
                let _ = u8::from_str_radix(blue, 16)?;
                let _ = u8::from_str_radix(alpha, 16)?;

                Ok(Color(Hex(String::from(value))))
            }
            _ => Err(ColorError::InvalidLength(hex_code.len())),
        }
    }

    /// Convert the hex color into rgba format
    ///
    /// # Example
    ///
    /// ```
    /// use agape_core::{Color,ColorError};
    ///
    /// fn main() -> Result<(),ColorError>{
    ///     let hex = Color::hex("#FFFFFF")?;
    ///     let rgba = hex.to_rgba();
    ///
    ///     assert_eq!(rgba.r(),255);
    ///     assert_eq!(rgba.g(),255);
    ///     assert_eq!(rgba.b(),255);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_rgba(&self) -> Color<Rgba> {
        let hex_code = &self.0.0.strip_prefix("#").unwrap();
        let (red, green, blue, alpha) = (
            &hex_code[0..2],
            &hex_code[2..4],
            &hex_code[4..6],
            &hex_code[6..8],
        );

        // TODO I don't know how I feel about these unwraps?
        // Hex codes should always be valid
        let r = u8::from_str_radix(red, 16).unwrap();
        let g = u8::from_str_radix(green, 16).unwrap();
        let b = u8::from_str_radix(blue, 16).unwrap();
        let a = u8::from_str_radix(alpha, 16).unwrap();

        Color::rgba(r, g, b, a)
    }

    pub fn as_str(&self) -> &str {
        self.0.0.as_str()
    }
}

impl std::fmt::Display for Color<Hex> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Color<Rgba> {
    fn default() -> Color<Rgba> {
        Color::rgba(0, 0, 0, 0)
    }
}

impl IntoColor<Rgba> for Color<Hex> {
    fn into_color(self) -> Color<Rgba> {
        self.to_rgba()
    }
}

impl IntoColor<Rgba> for Color<Rgba> {
    fn into_color(self) -> Color<Rgba> {
        self
    }
}

impl IntoColor<Rgba> for (u8, u8, u8, u8) {
    fn into_color(self) -> Color<Rgba> {
        let (r, g, b, a) = self;
        Color::rgba(r, g, b, a)
    }
}

impl IntoColor<Rgba> for u8 {
    /// Create a color with the same r,g,b value
    ///
    /// # Example
    ///
    /// ```
    /// use agape_core::{Color, IntoColor};
    ///
    /// let color = 27.into_color();
    ///
    /// assert_eq!(color.r(),27);
    /// assert_eq!(color.b(),27);
    /// assert_eq!(color.g(),27);
    /// assert_eq!(color.a(),100);
    /// ```
    fn into_color(self) -> Color<Rgba> {
        Color::rgba(self, self, self, 100)
    }
}

impl IntoColor<Rgba> for (u8, u8, u8) {
    fn into_color(self) -> Color<Rgba> {
        let (r, g, b) = self;

        Color::rgba(r, g, b, 100)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_colors_length() {
        let white = Color::hex("#FFFFFF").unwrap();
        let color = Color::hex("#00000000").unwrap();

        assert_eq!(white.to_string().len(), 9);
        assert_eq!(color.to_string().len(), 9);
    }

    #[test]
    fn rgba_alpha_clamped() {
        let color = Color::rgba(100, 100, 255, 255);
        assert_eq!(color.a(), 100);
    }

    #[test]
    fn invalid_hex_color() {
        assert_eq!(Color::hex("ffffff"), Err(ColorError::MissingHex));
        assert_eq!(Color::hex("#"), Err(ColorError::InvalidLength(0)));
        assert_eq!(Color::hex("#fff"), Err(ColorError::InvalidLength(3)));
    }
}
