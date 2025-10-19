use crate::{
    AxisAlignment, BoxConstraints, BoxSizing, IntrinsicSize, Layout, LayoutError, LayoutIter,
    Padding,
};
use agape_core::{GlobalId, Position, Size};
use std::ops::{Add, Mul};

// TODO make these private and add builder
/// A [`Layout`] that only has one child.
#[derive(Debug)]
pub struct BlockLayout {
    pub id: GlobalId,
    pub size: Size,
    pub position: Position,
    pub padding: Padding,
    pub intrinsic_size: IntrinsicSize,
    pub constraints: BoxConstraints,
    /// The main axis is the `x-axis`
    pub main_axis_alignment: AxisAlignment,
    /// The cross axis is the `y-axis`
    pub cross_axis_alignment: AxisAlignment,
    pub child: Box<dyn Layout>,
    errors: Vec<LayoutError>,
}

impl BlockLayout {
    pub fn new(child: Box<dyn Layout>) -> Self {
        Self {
            id: GlobalId::new(),
            size: Size::default(),
            padding: Padding::default(),
            position: Position::default(),
            intrinsic_size: IntrinsicSize::default(),
            constraints: BoxConstraints::default(),
            main_axis_alignment: AxisAlignment::default(),
            cross_axis_alignment: AxisAlignment::default(),
            errors: vec![],
            child,
        }
    }

    fn align_main_axis_start(&mut self) {
        let mut x_pos = self.position.x;
        x_pos += self.padding.left;
        self.child.set_x(x_pos);
    }

    /// Align the children on the main axis in the center
    fn align_main_axis_center(&mut self) {
        // TODO handle overflow
        let center_start = self.position.x + (self.size.width - self.child.size().width) / 2.0;
        self.child.set_x(center_start);
    }

    fn align_main_axis_end(&mut self) {
        // TODO: changed
        let mut x_pos = self.position.x + self.size.width;
        x_pos -= self.padding.right;

        self.child.set_x(x_pos);
    }

    fn align_cross_axis_start(&mut self) {
        // TODO: changed
        let y = self.position.y + self.padding.top;
        self.child.set_y(y);
    }

    fn align_cross_axis_center(&mut self) {
        // TODO handle overflow
        let y_pos = (self.size.height - self.child.size().height) / 2.0 + self.position.y;
        self.child.set_y(y_pos);
    }

    fn align_cross_axis_end(&mut self) {
        // TODO: changed
        self.child
            .set_y(self.position.y + self.size.height - self.padding.bottom);
    }
}

impl Layout for BlockLayout {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn size(&self) -> Size {
        self.size
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    fn position(&self) -> Position {
        self.position
    }

    fn children(&self) -> &[Box<dyn Layout>] {
        std::slice::from_ref(&self.child)
    }

    fn constraints(&self) -> BoxConstraints {
        self.constraints
    }

    fn intrinsic_size(&self) -> IntrinsicSize {
        self.intrinsic_size
    }

    fn set_max_height(&mut self, height: f32) {
        self.constraints.max_height = height;
    }

    fn set_max_width(&mut self, width: f32) {
        self.constraints.max_width = width;
    }

    fn set_min_height(&mut self, height: f32) {
        self.constraints.min_height = height;
    }

    fn set_min_width(&mut self, width: f32) {
        self.constraints.min_width = width;
    }

    fn collect_errors(&mut self) -> Vec<crate::LayoutError> {
        self.errors
            .drain(..)
            .chain(self.child.collect_errors())
            .collect::<Vec<_>>()
    }

    fn iter(&self) -> LayoutIter<'_> {
        LayoutIter { stack: vec![self] }
    }

    fn solve_min_constraints(&mut self) -> (f32, f32) {
        // The sum of the size of all the children with fixed sizes
        let mut min_size = Size::default();
        min_size += self.padding.bottom; // FIXME: unused

        // Solve the fixed size first
        if let BoxSizing::Fixed(width) = self.intrinsic_size.width {
            self.constraints.min_width = width;
        }

        if let BoxSizing::Fixed(height) = self.intrinsic_size.height {
            self.constraints.min_height = height;
        }

        if let BoxSizing::Fixed(width) = self.child.intrinsic_size().width {
            self.constraints.min_width = self.padding.right + self.padding.left + width;
        }

        if let BoxSizing::Fixed(height) = self.child.intrinsic_size().height {
            self.constraints.min_height = self.padding.top + self.padding.bottom + height;
        }

        let (min_width, min_height) = self.child.solve_min_constraints();

        // TODO: combine these
        match self.intrinsic_size.width {
            BoxSizing::Flex(_) => {
                self.constraints.min_width = self.padding.left + self.padding.right + min_width;
            }
            BoxSizing::Shrink => {
                self.constraints.min_width = self.padding.top + self.padding.bottom + min_width;
            }
            _ => {}
        }

        match self.intrinsic_size.height {
            BoxSizing::Flex(_) => {
                self.constraints.min_height = self.padding.top + self.padding.bottom + min_height;
            }
            BoxSizing::Shrink => {
                self.constraints.min_height = self.padding.top + self.padding.bottom + min_height;
            }
            _ => {}
        }

        (self.constraints.min_width, self.constraints.min_height)
    }

    fn solve_max_constraints(&mut self, space: Size) {
        // TODO: check
        let available_space = space - self.padding.left;

        match self.child.intrinsic_size().width {
            BoxSizing::Flex(_) => {
                self.child.set_max_width(available_space.width);
            }
            BoxSizing::Fixed(width) => {
                self.child.set_max_width(width);
            }
            BoxSizing::Shrink => {}
        }

        match self.child.intrinsic_size().height {
            BoxSizing::Flex(_) => {
                self.child.set_max_height(available_space.height);
            }
            BoxSizing::Fixed(height) => {
                self.child.set_max_height(height);
            }
            BoxSizing::Shrink => {}
        }

        self.child.solve_max_constraints(available_space);
    }

    fn update_size(&mut self) {
        match self.intrinsic_size.width {
            BoxSizing::Flex(_) => {
                self.size.width = self.constraints.max_width;
            }
            BoxSizing::Shrink => {
                self.size.width = self.constraints.min_width;
            }
            BoxSizing::Fixed(width) => {
                self.size.width = width;
            }
        }

        match self.intrinsic_size.height {
            BoxSizing::Flex(_) => {
                self.size.height = self.constraints.max_height;
            }
            BoxSizing::Shrink => {
                self.size.height = self.constraints.min_height;
            }
            BoxSizing::Fixed(height) => {
                self.size.height = height;
            }
        }

        self.child.update_size();
    }

    fn position_children(&mut self) {
        match self.main_axis_alignment {
            AxisAlignment::Start => self.align_main_axis_start(),
            AxisAlignment::Center => self.align_main_axis_center(),
            AxisAlignment::End => self.align_main_axis_end(),
        }

        match self.cross_axis_alignment {
            AxisAlignment::Start => self.align_cross_axis_start(),
            AxisAlignment::Center => self.align_cross_axis_center(),
            AxisAlignment::End => self.align_cross_axis_end(),
        }

        if self.child.position().x > self.position.x + self.size.width
            || self.child.position().y > self.position.y + self.size.height
        {
            self.errors.push(LayoutError::OutOfBounds {
                parent_id: self.id,
                child_id: self.child.id().to_owned(),
            });
        }
        self.child.position_children();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{EmptyLayout, solve_layout};

    #[test]
    fn test_shrink_sizing() {
        let window = Size::new(800.0, 800.0);
        let mut child = EmptyLayout::new();
        child.intrinsic_size.width = BoxSizing::Fixed(200.0);
        child.intrinsic_size.height = BoxSizing::Fixed(200.0);

        let mut root = BlockLayout::new(Box::new(child));
        root.padding = Padding::all(24.0);
        solve_layout(&mut root, window);

        let value = 24.0f32.mul_add(2.0, 200.0);
        assert_eq!(root.size(), Size::unit(value));
    }

    #[test]
    fn align_main_axis_start() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(Box::new(child));
        root.padding = Padding::new(0.0, 24.0, 24.24, 24.2);
        root.align_main_axis_start();
        let pos = root.child.position();
        assert_eq!(pos.x, root.position().x);
        root.padding.left = 10.0;
        root.align_main_axis_start();
        assert_eq!(root.child.position().x, root.position().x + 10.0);
    }

    #[test]
    fn align_main_axis_end() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(Box::new(child));
        root.size = Size::unit(200.0);
        root.align_main_axis_end();
        let pos = root.child.position();
        assert_eq!(pos.x, 200.0);
    }

    #[test]
    fn align_main_axis_center() {
        let mut child = EmptyLayout::new();
        child.size = Size::unit(250.0);
        let mut root = BlockLayout::new(Box::new(child));
        root.size = Size::unit(500.0);
        root.align_main_axis_end();
        let pos = root.child.position();
        assert_eq!(pos.x, 200.0);
    }

    #[test]
    fn align_cross_axis_start() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(Box::new(child));
        root.position.y = 50.0;
        root.align_cross_axis_start();
        let pos = root.child.position();
        assert_eq!(pos.y, 50.0);
    }

    #[test]
    fn align_cross_axis_start_uses_top_padding() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(Box::new(child));
        root.position.y = 50.0;
        root.padding.top = 10.0;
        root.align_cross_axis_start();
        let pos = root.child.position();
        assert_eq!(pos.y, 60.0);
    }

    #[test]
    fn align_main_axis_end_uses_right_padding() {
        let child = EmptyLayout::new();
        let mut root = BlockLayout::new(Box::new(child));
        root.padding = Padding::new(20.0, 50.0, 24.24, 24.2);
        root.size = Size::unit(200.0);
        root.align_main_axis_end();
        let pos = root.child.position();
        assert_eq!(pos.x, 200.0 - 50.0);
    }

    #[test]
    fn test_nested_shrink() {
        let window = Size::new(800.0, 800.0);

        let mut inner_child = EmptyLayout::new();
        inner_child.intrinsic_size.width = BoxSizing::Fixed(175.0);
        inner_child.intrinsic_size.height = BoxSizing::Fixed(15.0);

        let mut child = BlockLayout::new(Box::new(inner_child));
        child.padding = Padding::all(24.0);

        let mut root = BlockLayout::new(Box::new(child));

        solve_layout(&mut root, window);

        let inner_size = Size::new(175.0, 15.0);
        let child_size = inner_size + 24.0 * 2.0;

        assert_eq!(root.size(), child_size);
        assert_eq!(root.child.size(), child_size);
        assert_eq!(root.child.children()[0].size(), inner_size);
    }

    #[test]
    fn test_grow() {
        let window = Size::new(800.0, 800.0);
        let mut child = EmptyLayout::new();
        child.intrinsic_size.width = BoxSizing::Flex(1);
        child.intrinsic_size.height = BoxSizing::Flex(1);

        let mut root = BlockLayout::new(Box::new(child));
        root.intrinsic_size.width = BoxSizing::Flex(1);
        root.intrinsic_size.height = BoxSizing::Flex(1);
        root.padding = Padding::all(24.0);

        solve_layout(&mut root, window);

        let child_size = window - root.padding.left;
        assert_eq!(root.size(), window);
        assert_eq!(root.child.size(), child_size);
    }

    #[test]
    fn test_inner_grow() {
        let window = Size::new(800.0, 800.0);
        let mut inner_child = EmptyLayout::new();
        inner_child.intrinsic_size.width = BoxSizing::Flex(1);
        inner_child.intrinsic_size.height = BoxSizing::Flex(1);

        let mut child = BlockLayout::new(Box::new(inner_child));
        child.intrinsic_size.width = BoxSizing::Flex(1);
        child.intrinsic_size.height = BoxSizing::Flex(1);

        let mut root = BlockLayout::new(Box::new(child));
        root.intrinsic_size.width = BoxSizing::Flex(1);
        root.intrinsic_size.height = BoxSizing::Flex(1);
        root.padding = Padding::all(24.0);

        solve_layout(&mut root, window);

        let child_size = window - root.padding.left;
        assert_eq!(root.size(), window);
        assert_eq!(root.child.size(), child_size);
        assert_eq!(root.child.size(), root.child.children()[0].size());
    }
}
