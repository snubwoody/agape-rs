//! Core types
mod color;
mod position;
mod size;

pub use {color::*, position::*, size::*};

/// A global unique identifier
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Debug, Ord, Hash)]
pub struct GlobalId(u32);

impl GlobalId {
    pub fn new() -> Self {
        let id = rand::random();
        Self(id)
    }
}

impl Default for GlobalId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GlobalId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

/// Map value from one range to another. Any overflow or underflow is clipped to the min or max
///
/// # Example
/// ```
/// use agape_core::map;
/// let mapped_half = map(5.0,[0.0,10.0],[10.0,20.0]);
/// assert_eq!(mapped_half,15.0);
/// ```
pub fn map(mut value: f32, input_range: [f32; 2], output_range: [f32; 2]) -> f32 {
    if value > input_range[1] {
        value = input_range[1]
    } else if value < input_range[0] {
        value = input_range[0]
    }

    let scale = (output_range[1] - output_range[0]) / (input_range[1] - input_range[0]);
    let offset = input_range[0] * (scale) + output_range[0];

    value * scale + offset
}

#[cfg(test)]
mod test {
    use crate::GlobalId;
    use std::collections::HashSet;

    #[test]
    fn global_ids_are_unique() {
        // Just checking that the ids are unique (enough)
        // in case it's ever changed

        let mut ids = HashSet::new();
        for _ in 0..10_000 {
            assert!(ids.insert(GlobalId::new()));
        }
    }
}
