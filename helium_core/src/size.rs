use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use winit::dpi::PhysicalSize;

/// Anything with a width and a height
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn scale(&mut self, factor: f32) {
        self.width *= factor;
        self.height *= factor;
    }

    pub fn set(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

impl From<PhysicalSize<u32>> for Size {
    fn from(size: PhysicalSize<u32>) -> Self {
        Self {
            width: size.width as f32,
            height: size.height as f32,
        }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            width: self.width + other.width,
            height: self.height + other.height,
        };
    }
}

impl<N> AddAssign<N> for Size
where
    N: Into<f32>,
{
    fn add_assign(&mut self, other: N) {
        let other = other.into();
        *self = Self {
            width: self.width + other,
            height: self.height + other,
        };
    }
}

impl<N> SubAssign<N> for Size
where
    N: Into<f32>,
{
    fn sub_assign(&mut self, other: N) {
        let other = other.into();
        *self = Self {
            width: self.width - other,
            height: self.height - other,
        };
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl<N> Add<N> for Size
where
    N: Into<f32>,
{
    type Output = Self;

    fn add(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Self {
            width: self.width + rhs,
            height: self.height + rhs,
        }
    }
}

impl<N> Sub<N> for Size
where
    N: Into<f32>,
{
    type Output = Self;

    fn sub(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Self {
            width: self.width - rhs,
            height: self.height - rhs,
        }
    }
}

impl Sub for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl<N> Mul<N> for Size
where
    N: Into<f32>,
{
    type Output = Self;
    fn mul(self, rhs: N) -> Self::Output {
        let rhs: f32 = rhs.into();
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl<N> Div<N> for Size
where
    N: Into<f32>,
{
    type Output = Self;
    fn div(self, rhs: N) -> Self::Output {
        let rhs: f32 = rhs.into();
        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl From<(u32,u32)> for Size{
	fn from(value: (u32,u32)) -> Self {
		Self { width: value.0 as f32, height: value.1 as f32}
	}
}
