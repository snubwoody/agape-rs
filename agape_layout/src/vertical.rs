use crate::{
    AxisAlignment, BoxConstraints, BoxSizing, IntrinsicSize, Layout, LayoutError, LayoutIter,
    error::OverflowAxis,
};
use agape_core::{GlobalId, Position, Size};
// TODO maybe make some items private
// TODO if min width is larger than max width then it's an overflow
/// A [`Layout`] that arranges it's children vertically.
#[derive(Default, Debug)]
pub struct VerticalLayout {
    pub id: GlobalId,
    pub size: Size,
    pub position: Position,
    pub spacing: u32,
    pub padding: u32,
    pub scroll_offset: f32,
    pub intrinsic_size: IntrinsicSize,
    // TODO i'm thinking of adding user constraints as well so that people can define their own
    // constraints
    pub children: Vec<Box<dyn Layout>>,
    /// The main axis is the `y-axis`
    pub main_axis_alignment: AxisAlignment,
    /// The cross axis is the `x-axis`
    pub cross_axis_alignment: AxisAlignment,
    pub constraints: BoxConstraints,
    pub errors: Vec<crate::LayoutError>,
}

impl VerticalLayout {
    /// Creates a new [`VerticalLayout`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a child [`Layout`] to the `VerticalLayout`
    pub fn add_child(&mut self, child: impl Layout + 'static) {
        self.children.push(Box::new(child));
    }

    pub fn add_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item: Layout + 'static>,
    {
        for child in children {
            self.children.push(Box::new(child));
        }
    }

    /// Returns `true` if a [`VerticalLayout`]'s children are overflowing.
    pub fn overflow(&self) -> bool {
        self.main_axis_overflow() || self.cross_axis_overflow()
    }

    /// Returns `true` if a [`VerticalLayout`]'s children are overflowing it's main-axis
    /// (y-axis).
    pub fn main_axis_overflow(&self) -> bool {
        self.errors
            .contains(&LayoutError::overflow(self.id, OverflowAxis::MainAxis))
    }

    /// Returns `true` if a [`VerticalLayout`]'s children are overflowing it's cross-axis
    /// (x-axis).
    pub fn cross_axis_overflow(&self) -> bool {
        self.errors
            .contains(&LayoutError::overflow(self.id, OverflowAxis::CrossAxis))
    }

    fn fixed_size_sum(&self) -> Size {
        let mut sum = Size::default();

        for child in self.children.iter() {
            if let BoxSizing::Fixed(width) = child.intrinsic_size().width {
                sum.width = sum.width.max(width);
            }

            match child.intrinsic_size().height {
                BoxSizing::Fixed(height) => {
                    sum.height += height;
                }
                BoxSizing::Shrink => {
                    sum.height += child.constraints().min_height;
                }
                _ => {}
            }
        }

        sum
    }

    pub fn scroll(&mut self, offset: f32) {
        self.scroll_offset += offset;
    }

    /// Align the children on the main axis at the start
    fn align_main_axis_start(&mut self) {
        let mut y = self.position.y;
        y += self.padding as f32;

        for child in &mut self.children {
            child.set_y(y);
            y += child.size().height + self.spacing as f32;
        }
    }

    /// Align the children on the main axis in the center
    fn align_main_axis_center(&mut self) {
        // TODO handle overflow
        let mut height_sum = self
            .children
            .iter()
            .map(|child| child.size().height)
            .sum::<f32>();

        height_sum += (self.spacing * (self.children.len() as u32 - 1)) as f32;
        let mut center_start = self.position.y + (self.size.height - height_sum) / 2.0;

        for child in &mut self.children {
            child.set_y(center_start);
            center_start += child.size().height + self.spacing as f32;
        }
    }

    fn align_main_axis_end(&mut self) {
        let mut y = self.position.y + self.size.height;
        y -= self.padding as f32;

        for child in self.children.iter_mut().rev() {
            child.set_y(y);
            y -= child.size().height - self.spacing as f32;
        }
    }

    fn align_cross_axis_start(&mut self) {
        let x = self.position.x + self.padding as f32;
        for child in &mut self.children {
            child.set_x(x);
        }
    }

    fn align_cross_axis_center(&mut self) {
        for child in &mut self.children {
            // TODO handle overflow
            let x_pos = (self.size.width - child.size().width) / 2.0 + self.position.x;
            child.set_x(x_pos);
        }
    }

    fn align_cross_axis_end(&mut self) {
        for child in &mut self.children {
            child.set_x(self.position.x + self.size.width - self.padding as f32);
        }
    }
}

impl Layout for VerticalLayout {
    fn id(&self) -> GlobalId {
        self.id
    }

    fn position(&self) -> Position {
        self.position
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

    fn size(&self) -> Size {
        self.size
    }

    fn children(&self) -> &[Box<dyn Layout>] {
        self.children.as_slice()
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
            .chain(
                self.children
                    .iter_mut()
                    .flat_map(|child| child.collect_errors())
                    .collect::<Vec<_>>(),
            )
            .collect::<Vec<_>>()
    }

    fn iter(&self) -> crate::LayoutIter {
        LayoutIter { stack: vec![self] }
    }

    fn solve_min_constraints(&mut self) -> (f32, f32) {
        let mut child_constraint_sum = Size::default();
        for child in &mut self.children {
            let (min_width, min_height) = child.solve_min_constraints();
            child_constraint_sum.height += min_height;
            child_constraint_sum.height += self.spacing as f32;
            child_constraint_sum.width = child_constraint_sum.width.max(min_width);
        }
        child_constraint_sum += self.padding as f32 * 2.0;

        // TODO i think im supposed to calculate the min constraints of the children as well
        match self.intrinsic_size.width {
            BoxSizing::Fixed(width) => {
                self.constraints.min_width = width;
            }
            BoxSizing::Flex(_) => {
                // TODO maybe set the min constraints to either 0 or the size of the children
                self.constraints.min_width = child_constraint_sum.width;
            }
            BoxSizing::Shrink => {
                self.constraints.min_width = child_constraint_sum.width;
            }
        }

        match self.intrinsic_size.height {
            BoxSizing::Fixed(height) => {
                self.constraints.min_height = height;
            }
            BoxSizing::Flex(_) => {
                self.constraints.min_height = child_constraint_sum.height;
            }
            BoxSizing::Shrink => {
                self.constraints.min_height = child_constraint_sum.height;
            }
        }

        (self.constraints.min_width, self.constraints.min_height)
    }

    fn solve_max_constraints(&mut self, _space: Size) {
        // Sum up all the flex factors
        let flex_total: u8 = self
            .children
            .iter()
            .filter_map(|child| {
                if let BoxSizing::Flex(factor) = child.intrinsic_size().height {
                    Some(factor)
                } else {
                    None
                }
            })
            .sum();

        let mut available_height;
        match self.intrinsic_size.height {
            BoxSizing::Shrink => {
                available_height = self.constraints.min_height;
                available_height -= self.fixed_size_sum().height;
            }
            BoxSizing::Fixed(_) | BoxSizing::Flex(_) => {
                available_height = self.constraints.max_height;
                available_height -= self.padding as f32 * 2.0;
                available_height -= self.fixed_size_sum().height;
            }
        }

        let mut available_width;
        match self.intrinsic_size.width {
            BoxSizing::Shrink => available_width = self.constraints.min_width,
            BoxSizing::Fixed(_) | BoxSizing::Flex(_) => {
                available_width = self.constraints.max_width;
                available_width -= self.padding as f32 * 2.0;
            }
        }

        if !self.children.is_empty() {
            // Add the spacing between layouts
            for _ in 0..self.children.len() - 1 {
                available_height -= self.spacing as f32;
            }
        }

        for child in self.children.iter_mut() {
            match child.intrinsic_size().width {
                BoxSizing::Flex(_) => {
                    child.set_max_width(available_width);
                }
                BoxSizing::Shrink => {
                    child.set_max_width(child.constraints().min_width);
                }
                BoxSizing::Fixed(width) => {
                    child.set_max_width(width);
                }
            }

            match child.intrinsic_size().height {
                BoxSizing::Flex(factor) => {
                    let grow_factor = factor as f32 / flex_total as f32;
                    child.set_max_height(grow_factor * available_height);
                }
                BoxSizing::Fixed(height) => {
                    child.set_max_height(height);
                }
                BoxSizing::Shrink => {}
            }

            // TODO not using size anymore
            child.solve_max_constraints(Size::default());
        }
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
                // TODO maybe set the min constrains?
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
                // TODO maybe set the min constrains?
                self.size.height = height;
            }
        }

        for child in &mut self.children {
            child.update_size();
        }

        // TODO check for padding and spacing
        let width_sum: f32 = self.children.iter().map(|child| child.size().width).sum();
        let mut height_sum = (self.padding * 2) as f32;
        for (i, child) in self.children.iter().enumerate() {
            height_sum += child.size().height;
            if i != self.children.len() - 1 {
                height_sum += self.spacing as f32;
            }
        }

        let main_axis_error = LayoutError::overflow(self.id, OverflowAxis::MainAxis);
        let cross_axis_error = LayoutError::overflow(self.id, OverflowAxis::CrossAxis);

        // Prevent duplicate errors
        if !self.errors.contains(&cross_axis_error) && width_sum > self.size.width {
            self.errors.push(cross_axis_error);
        }

        if !self.errors.contains(&main_axis_error) && height_sum > self.size.height {
            self.errors.push(main_axis_error);
        }
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

        let overflow = self.main_axis_overflow();

        for child in &mut self.children {
            // Only scroll if children are overflowing
            if overflow {
                let y = child.position().y;
                child.set_y(y + self.scroll_offset);
            }

            if child.position().y > self.position.y + self.size.height {
                self.errors.push(LayoutError::OutOfBounds {
                    parent_id: self.id,
                    child_id: child.id().to_owned(),
                });
            }
            child.position_children();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{BlockLayout, EmptyLayout, LayoutSolver};

    #[test]
    fn overflow_error() {
        let window = Size::unit(500.0);

        let mut root = VerticalLayout::new();
        root.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(0.0),
            height: BoxSizing::Fixed(0.0),
        };

        let mut child = EmptyLayout::new();
        child.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(200.0),
            height: BoxSizing::Fixed(200.0),
        };

        root.add_child(child);

        LayoutSolver::solve(&mut root, window);
        assert!(root.main_axis_overflow());
        assert!(root.cross_axis_overflow());
    }

    #[test]
    fn cross_axis_overflow() {
        let window = Size::unit(500.0);

        let mut root = VerticalLayout::new();
        root.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(0.0),
            ..Default::default()
        };

        let mut child = EmptyLayout::new();
        child.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(200.0),
            ..Default::default()
        };

        root.add_child(child);

        LayoutSolver::solve(&mut root, window);
        dbg!(&root);
        assert!(!root.main_axis_overflow());
        assert!(root.cross_axis_overflow());
    }

    #[test]
    fn main_axis_overflow() {
        let window = Size::unit(500.0);

        let mut root = VerticalLayout::new();
        root.intrinsic_size = IntrinsicSize {
            height: BoxSizing::Fixed(0.0),
            ..Default::default()
        };

        let mut child = EmptyLayout::new();
        child.intrinsic_size = IntrinsicSize {
            height: BoxSizing::Fixed(200.0),
            ..Default::default()
        };

        root.add_child(child);

        LayoutSolver::solve(&mut root, window);
        assert!(root.main_axis_overflow());
        assert!(!root.cross_axis_overflow());
    }
    #[test]
    fn include_spacing_and_padding_main_axis_overflow() {
        let window = Size::unit(500.0);

        let mut root = VerticalLayout::new();
        root.spacing = 20;
        root.padding = 20;
        root.intrinsic_size = IntrinsicSize {
            height: BoxSizing::Fixed(200.0),
            ..Default::default()
        };

        let mut child = EmptyLayout::new();
        child.intrinsic_size = IntrinsicSize {
            height: BoxSizing::Fixed(180.0),
            ..Default::default()
        };

        root.add_child(child);

        LayoutSolver::solve(&mut root, window);
        assert!(root.main_axis_overflow());
        assert!(!root.cross_axis_overflow());
    }

    #[test]
    fn no_duplicate_overflow_error() {
        let window = Size::unit(500.0);

        let mut root = VerticalLayout::new();
        root.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(0.0),
            ..Default::default()
        };

        let mut child = EmptyLayout::new();
        child.intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(200.0),
            height: BoxSizing::Fixed(200.0),
        };

        root.add_child(child);

        LayoutSolver::solve(&mut root, window);
        LayoutSolver::solve(&mut root, window);
        LayoutSolver::solve(&mut root, window);
        LayoutSolver::solve(&mut root, window);

        assert_eq!(root.errors.len(), 1);
    }

    #[test]
    fn vertical_layout() {
        let window = Size::new(800.0, 800.0);
        let mut root = VerticalLayout::new();
        let mut child_1 = VerticalLayout::new();
        let mut child_2 = VerticalLayout::new();

        child_1.intrinsic_size.width = BoxSizing::Fixed(400.0);
        child_1.intrinsic_size.height = BoxSizing::Fixed(200.0);

        child_2.intrinsic_size.width = BoxSizing::Fixed(500.0);
        child_2.intrinsic_size.height = BoxSizing::Fixed(350.0);

        root.add_child(child_1);
        root.add_child(child_2);

        LayoutSolver::solve(&mut root, window);

        assert_eq!(root.size(), Size::new(500.0, 550.0));

        assert_eq!(root.children()[0].size(), Size::new(400.0, 200.0));

        assert_eq!(root.children()[1].size(), Size::new(500.0, 350.0));
    }

    /// Padding should still be applied when a `VerticalLayout` is empty to ensure
    /// consistency in the overall layout. It also preserves the structure
    /// if layouts are added later on
    #[test]
    fn padding_applied_when_empty() {
        let mut empty = VerticalLayout {
            padding: 23,
            ..Default::default()
        };
        LayoutSolver::solve(&mut empty, Size::new(200.0, 200.0));

        assert_eq!(empty.size, Size::new(23.0 * 2.0, 23.0 * 2.0));
    }

    /// Spacing should not be applied when a [`VerticalLayout`] is empty even though the
    /// `padding` should, as spacing is the space in between `layouts` so there is no consistency
    /// being lost, it would actually mess up with the size as it would be added with the `padding`,
    #[test]
    fn spacing_not_applied_when_empty() {
        let mut empty = VerticalLayout {
            spacing: 50,
            ..Default::default()
        };
        LayoutSolver::solve(&mut empty, Size::new(200.0, 200.0));

        assert_eq!(empty.size, Size::default());
    }

    #[test]
    fn flex_sizing() {
        let window = Size::new(800.0, 800.0);
        let mut root = VerticalLayout::new();
        let mut child_1 = VerticalLayout::new();
        let mut child_2 = VerticalLayout::new();

        child_1.intrinsic_size.width = BoxSizing::Flex(1);
        child_1.intrinsic_size.height = BoxSizing::Flex(1);

        child_2.intrinsic_size.width = BoxSizing::Flex(1);
        child_2.intrinsic_size.height = BoxSizing::Flex(1);

        root.intrinsic_size.width = BoxSizing::Flex(1);
        root.intrinsic_size.height = BoxSizing::Flex(1);

        root.add_child(child_1);
        root.add_child(child_2);

        LayoutSolver::solve(&mut root, window);

        let child_size = Size::new(800.0, 400.0);
        assert_eq!(root.size(), window);
        assert_eq!(root.children()[0].size(), child_size);
        assert_eq!(root.children()[1].size(), child_size);
    }

    #[test]
    fn flex_with_shrink() {
        let window = Size::new(800.0, 800.0);
        let padding = 24;
        let spacing = 45;

        let mut inner_child = EmptyLayout::new();
        inner_child.intrinsic_size.width = BoxSizing::Fixed(250.0);
        inner_child.intrinsic_size.height = BoxSizing::Fixed(250.0);

        let mut child_1 = BlockLayout::new(Box::new(inner_child));
        child_1.padding = padding;

        let mut child_2 = EmptyLayout::new();
        child_2.intrinsic_size.width = BoxSizing::Flex(1);
        child_2.intrinsic_size.height = BoxSizing::Flex(1);

        let mut root = VerticalLayout::new();
        root.intrinsic_size.height = BoxSizing::Flex(1);
        root.padding = padding;
        root.spacing = spacing;
        root.add_child(child_1);
        root.add_child(child_2);

        LayoutSolver::solve(&mut root, window);

        let mut child_1_size = Size::new(250.0, 250.0);
        child_1_size += (padding * 2) as f32;

        let mut root_size = Size::new(0.0, 800.0);
        root_size.width += child_1_size.width;
        root_size.width += (padding * 2) as f32;

        // I feel like the math is slightly wrong due to padding
        let mut child_2_size = Size {
            width: root_size.width,
            height: root_size.height,
        };
        child_2_size.height -= child_1_size.height;
        child_2_size.height -= spacing as f32;
        child_2_size.height -= (padding * 2) as f32;

        assert_eq!(root.size(), root_size);
        assert_eq!(root.children[0].size(), child_1_size);
        assert_eq!(root.children[1].size(), child_2_size);
    }

    // TODO test flex grow inside flex shrink
    #[test]
    fn flex_factor() {
        let window = Size::new(800.0, 400.0);
        let mut node = VerticalLayout::new();
        let mut child_node_1 = VerticalLayout::new();
        let mut child_node_2 = VerticalLayout::new();

        child_node_1.intrinsic_size.width = BoxSizing::Flex(1);
        child_node_1.intrinsic_size.height = BoxSizing::Flex(1);

        child_node_2.intrinsic_size.width = BoxSizing::Flex(3);
        child_node_2.intrinsic_size.height = BoxSizing::Flex(3);

        node.intrinsic_size.width = BoxSizing::Flex(1);
        node.intrinsic_size.height = BoxSizing::Flex(1);

        node.add_child(child_node_1);
        node.add_child(child_node_2);

        LayoutSolver::solve(&mut node, window);

        let flex_1_height = 1.0 / 4.0 * window.height;
        // The two children should both be half the size
        assert_eq!(
            node.children()[0].size(),
            Size::new(window.width, flex_1_height)
        );
        assert_eq!(
            node.children()[0].size().width,
            node.children()[1].size().width,
        );
        assert!(node.children()[1].size().height == 3.0 * node.children()[0].size().height);
        assert!(node.children()[1].size().width != 3.0 * node.children()[0].size().width);
    }
}
