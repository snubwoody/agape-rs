use crate::size::Size;
use std::ops::{AddAssign, SubAssign};
use winit::dpi::PhysicalPosition;

/// Represents the position of any structure
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Translate the position by `x` and `y` amount.
    /// # Example
    /// ```
    /// use helium_core::position::Position;
    ///
    /// let mut position = Position::new(0.0,0.0);
    /// position.translate(40.0,100.0);
    ///
    /// assert_eq!(Position::new(40.0,100.0),position);
    /// ```
    pub fn translate(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    /// Set the position
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<I> AddAssign<I> for Position
where
    f32: AddAssign<I>,
    I: Copy,
{
    fn add_assign(&mut self, rhs: I) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<I> SubAssign<I> for Position
where
    f32: SubAssign<I>,
    I: Copy,
{
    fn sub_assign(&mut self, rhs: I) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl From<PhysicalPosition<f64>> for Position {
    fn from(position: PhysicalPosition<f64>) -> Self {
        Self {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

/// The bounds of any object that has a [`Size`]
/// and [`Position`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Bounds {
    pub x: [f32; 2],
    pub y: [f32; 2],
}

impl Bounds {
    pub fn new(position: Position, size: Size) -> Self {
        Self {
            x: [position.x, position.x + size.width],
            y: [position.y, position.y + size.height],
        }
    }

    /// Check if a [`Position`] is within the [`Bounds`].
    ///
    /// # Example
    /// ```
    ///     use helium_core::{position::{Position,Bounds},size::Size};
    ///
    ///     let size = Size::new(250.0,100.0);
    ///     let position = Position::new(10.0,0.0);
    ///
    ///     let bounds = Bounds::new(&position,&size);
    ///
    ///     assert!(bounds.within(&Position::new(50.0,45.5)));
    /// ```
    pub fn within(&self, position: &Position) -> bool {
        // TODO change the name of this to has and move within to position
        if position.x > self.x[0]
            && position.x < self.x[1]
            && position.y > self.y[0]
            && position.y < self.y[1]
        {
            return true;
        }

        false
    }
}
