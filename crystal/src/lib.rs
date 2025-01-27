//! This is the crate that manages all the helium layouts, at a basic level every layout
//! node must return a size and position so that other layouts can arrange themselves
//! accordingly.
//!
//! # Getting started
//! ```
//! use crystal::{EmptyLayout,HorizontalLayout,LayoutSolver,Size};
//!
//! let mut root = HorizontalLayout::new();
//! root.add_children([EmptyLayout::new(),EmptyLayout::new()]);
//!
//! // Pass in the root layout and the window size
//! // The layout solver returns any errors that occured, such as layout overflow
//! let _ = LayoutSolver::solve(&mut root,Size::new(500.0,500.0));
//! ```
//!
//! This layout engine is based on the idea that a [`Layout`] can only have one of three
//! different intrinsic sizes, known as [`BoxSizing`]
//! - It wants to be as large as possible, usually filling the parent, this is the
//! `BoxSizing::Flex(u8)` variant. It also has a flex factor which can be used to
//! control how much space it takes relative it's to sibling `Layouts`.
//! - It wants to be as small as possible, usually fitting it's children, this is the
//! `BoxSizing::Shrink` variant
//! - It wants to be a certain fixed size, this is the `BoxSizing::Fixed` variant.
//!
//! `crystal` uses `layouts` to perform calculations, a layout is anything which implements
//! the [`Layout`] trait. Currently there are 4 distinct types of [`Layout`]
//!
//! - [`HorizontalLayout`]: Arranges children horizontally
//! - [`VerticalLayout`]: Arranges children verically
//! - [`BlockLayout`]: A layout with a single child
//! - [`EmptyLayout`]: A layout with no children, commonly used for things like
//! text and images.

mod block;
mod empty;
mod error;
mod horizontal;
mod vertical;
pub use block::BlockLayout;
pub use empty::EmptyLayout;
pub use error::LayoutError;
pub use helium_core::{position::Position, size::Size};
pub use horizontal::HorizontalLayout;
use std::fmt::Debug;
pub use vertical::VerticalLayout;

pub struct LayoutSolver;

impl LayoutSolver {
    /// Calculates the layout of all the layout nodes
    pub fn solve(root: &mut dyn Layout, window_size: Size) -> Vec<crate::LayoutError> {
        root.set_max_width(window_size.width);
        root.set_max_height(window_size.height);

        // It's important that the min constraints are solved before the max constraints
        // because the min constraints are used in calculating max constraints
        let _ = root.solve_min_constraints();
        root.solve_max_contraints(window_size);
        root.update_size();
        root.position_children();

        // TODO add a push error function that checks for equality so that we don't have duplicate errors
        // or maybe just clear the error stack every frame
        root.collect_errors()
    }
}

pub trait Layout: Debug + Send + Sync {
    /// Solve the minimum constraints of each layout node recursively, if
    /// the node has an instrinsic size of `Fixed` then it's minimum size is
    /// set to the fixed values, if it's intrinsic size is set to `Shrink` then
    /// it get's the min constraints of it's children bubbling them up the layout
    /// tree.
    fn solve_min_constraints(&mut self) -> (f32, f32);

    /// Solve the max constraints for the children and pass them down the tree
    fn solve_max_contraints(&mut self, space: Size);

    fn position_children(&mut self);

    /// Update the size of every [`LayoutNode`] based on it's size and constraints.
    fn update_size(&mut self);

    /// Collect all the errors from the error stack
    fn collect_errors(&mut self) -> Vec<LayoutError>;

    /// Sort the children based on their intrinsic sizing, [`HorizontalLayout`]'s are ordered
    /// based on the children's `intrinsic width` and [`VerticalLayout`]'s are ordered based on their
    /// children's `intrinsic height`.
    #[deprecated = "Was not working as intended"]
    fn sort_children(&mut self); // FIXME remove this

    fn id(&self) -> &str;
    fn constraints(&self) -> BoxContraints;
    fn intrinsic_size(&self) -> IntrinsicSize;
    fn size(&self) -> Size;
    fn position(&self) -> Position;
    fn children(&self) -> &[Box<dyn Layout>];

    fn set_max_width(&mut self, width: f32);
    fn set_max_height(&mut self, height: f32);
    fn set_min_width(&mut self, width: f32);
    fn set_min_height(&mut self, height: f32);
    fn set_position(&mut self, position: Position);
    fn set_x(&mut self, x: f32);
    fn set_y(&mut self, y: f32);

    fn iter(&self) -> LayoutIter;

    /// Get a [`Layout`] by it's `id`.
    fn get(&self, id: &str) -> Option<&dyn Layout> {
        for layout in self.iter() {
            if layout.id() == id {
                return Some(layout);
            }
        }
        None
    }
}

pub struct LayoutIter<'a> {
    stack: Vec<&'a dyn Layout>, // TODO unnecessary box and maybe make it a provided trait
}

impl<'a> Iterator for LayoutIter<'a> {
    type Item = &'a dyn Layout;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(layout) = self.stack.pop() {
            let children = layout.children();

            let k = children.iter().map(|child| {
                // Type gymnastics indeed
                &*child.as_ref()
            });

            self.stack.extend(k.rev());
            return Some(layout);
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutType {
    #[default]
    Block,
    Horizontal,
    Vertical,
}

/// Decribes the size a [`Layout`] will try to be.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub enum BoxSizing {
    Fixed(f32),
    /// Tries to be as small as possible
    #[default]
    Shrink,
    /// Tries to be as big as possible, the behaviour of the flex factor is
    /// dependant on the type of layout.
    Flex(u8),
}

/// Describes how a [`Layout`] should arrange it's children
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum AxisAlignment {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BoxContraints {
    pub max_width: f32,
    pub max_height: f32,
    pub min_height: f32,
    pub min_width: f32,
}

impl BoxContraints {
    pub fn new() -> Self {
        Self::default()
    }
}

/// This is the size that a [`Widget`] will try to be,  
/// the actual final size is dependant on the space
/// available.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct IntrinsicSize {
    // TODO does this really need to be a seperate struct?
    pub width: BoxSizing,
    pub height: BoxSizing,
}

#[cfg(test)]
mod test {
    use super::*;

    // FIXME
    #[test]
    fn test_horizontal_and_empty_layout() {
        // TODO split this into a smaller function that tests flex layouts have 0 size
        let window = Size::new(1000.0, 1000.0);

        let mut child_1 = EmptyLayout::new();
        child_1.intrinsic_size.width = BoxSizing::Fixed(250.0);
        child_1.intrinsic_size.height = BoxSizing::Flex(1);

        let mut child_2 = EmptyLayout::new();
        child_2.intrinsic_size.width = BoxSizing::Flex(1);
        child_2.intrinsic_size.height = BoxSizing::Fixed(20.0);

        let mut child_3 = EmptyLayout::new();
        child_3.intrinsic_size.height = BoxSizing::Fixed(250.0);

        let mut root = HorizontalLayout::new();
        root.add_children([child_1, child_2, child_3]);

        LayoutSolver::solve(&mut root, window);

        assert_eq!(root.size(), Size::new(250.0, 250.0));
        assert_eq!(root.children[0].size(), Size::new(250.0, 250.0));
        assert_eq!(root.children[1].size(), Size::new(0.0, 20.0));
        assert_eq!(root.children[2].size(), Size::new(0.0, 250.0));
    }
}
