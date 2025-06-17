use crystal::{
    BlockLayout, BoxSizing, EmptyLayout, HorizontalLayout, Layout, LayoutSolver, Size,
};

#[test]
fn test_horizontal_layout() {
    let window = Size::new(800.0, 800.0);
    let mut root = HorizontalLayout::new();
    let mut child_1 = HorizontalLayout::new();
    let mut child_2 = HorizontalLayout::new();

    child_1.intrinsic_size.width = BoxSizing::Fixed(400.0);
    child_1.intrinsic_size.height = BoxSizing::Fixed(200.0);

    child_2.intrinsic_size.width = BoxSizing::Fixed(500.0);
    child_2.intrinsic_size.height = BoxSizing::Fixed(350.0);

    root.add_child(child_1);
    root.add_child(child_2);

    LayoutSolver::solve(&mut root, window);

    assert_eq!(root.size(), Size::new(900.0, 350.0));

    assert_eq!(root.children()[0].size(), Size::new(400.0, 200.0));

    assert_eq!(root.children()[1].size(), Size::new(500.0, 350.0));
}

#[test]
fn horizontal_and_empty_layout() {
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

#[test]
fn test_flex_sizing() {
    let window = Size::new(800.0, 800.0);
    let mut root = HorizontalLayout::new();
    let mut child_1 = HorizontalLayout::new();
    let mut child_2 = HorizontalLayout::new();

    child_1.intrinsic_size.width = BoxSizing::Flex(1);
    child_1.intrinsic_size.height = BoxSizing::Flex(1);

    child_2.intrinsic_size.width = BoxSizing::Flex(1);
    child_2.intrinsic_size.height = BoxSizing::Flex(1);

    root.intrinsic_size.width = BoxSizing::Flex(1);
    root.intrinsic_size.height = BoxSizing::Flex(1);

    root.add_child(child_1);
    root.add_child(child_2);

    LayoutSolver::solve(&mut root, window);
    let child_size = Size::new(400.0, 800.0);
    assert_eq!(root.size(), window);
    assert_eq!(root.children()[0].size(), child_size);
    assert_eq!(root.children()[1].size(), child_size);
}

#[test]
fn test_flex_with_shrink() {
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

    let mut root = HorizontalLayout::new();
    root.intrinsic_size.width = BoxSizing::Flex(1);
    root.padding = padding;
    root.spacing = spacing;
    root.add_child(child_1);
    root.add_child(child_2);

    LayoutSolver::solve(&mut root, window);

    let mut child_1_size = Size::new(250.0, 250.0);
    child_1_size += (padding * 2) as f32;

    let mut root_size = Size::new(800.0, 250.0);
    root_size.height += (padding * 4) as f32; // Add the padding for child_1 and for the root

    let mut child_2_size = Size::new(window.width, child_1_size.height);
    child_2_size.width -= child_1_size.width;
    child_2_size.width -= spacing as f32;
    child_2_size.width -= (padding * 2) as f32;
    child_2_size.height += (padding * 2) as f32;

    assert_eq!(root.size(), root_size);
    assert_eq!(root.children[0].size(), child_1_size);
    assert_eq!(root.children[1].size(), child_2_size);
}

#[test]
fn test_flex_with_fixed() {
    let window = Size::new(800.0, 800.0);
    let padding = 24;
    let spacing = 45;

    let mut child_1 = EmptyLayout::new();
    child_1.intrinsic_size.width = BoxSizing::Fixed(250.0);
    child_1.intrinsic_size.height = BoxSizing::Fixed(250.0);

    let mut child_2 = EmptyLayout::new();
    child_2.intrinsic_size.width = BoxSizing::Flex(1);
    child_2.intrinsic_size.height = BoxSizing::Flex(2);

    let mut child_3 = EmptyLayout::new();
    child_3.intrinsic_size.width = BoxSizing::Flex(4);
    child_3.intrinsic_size.height = BoxSizing::Flex(4);

    let mut root = HorizontalLayout::new();
    root.intrinsic_size.width = BoxSizing::Flex(1);
    root.intrinsic_size.height = BoxSizing::Flex(1);
    root.padding = padding;
    root.spacing = spacing;
    root.add_children([child_1, child_2, child_3]);

    LayoutSolver::solve(&mut root, window);
    dbg!(&root);

    let mut space = window;
    space -= (padding * 2) as f32;
    space -= (spacing * 2) as f32;
    space.width -= 250.0;

    assert_eq!(root.children[1].size().width, 1.0 / 5.0 * space.width);
    assert_eq!(root.children[2].size().width, 4.0 / 5.0 * space.width);
    assert_eq!(
        root.children[1].size().height,
        window.height - (padding * 2) as f32
    );
}

#[test]
fn test_flex_factor() {
    let window = Size::new(800.0, 400.0);
    let mut node = HorizontalLayout::new();
    let mut child_node_1 = HorizontalLayout::new();
    let mut child_node_2 = HorizontalLayout::new();

    child_node_1.intrinsic_size.width = BoxSizing::Flex(1);
    child_node_1.intrinsic_size.height = BoxSizing::Flex(1);

    child_node_2.intrinsic_size.width = BoxSizing::Flex(3);
    child_node_2.intrinsic_size.height = BoxSizing::Flex(3);

    node.intrinsic_size.width = BoxSizing::Flex(1);
    node.intrinsic_size.height = BoxSizing::Flex(1);

    node.add_child(child_node_1);
    node.add_child(child_node_2);

    LayoutSolver::solve(&mut node, window);

    let flex_1_width = 1.0 / 4.0 * window.width;
    // The two children should both be half the size
    assert_eq!(node.children()[0].size().width, flex_1_width);
    assert_eq!(node.children()[0].size().height, 400.0);
    assert_eq!(
        node.children()[0].size().height,
        node.children()[1].size().height,
    );
    assert!(node.children()[1].size().width == 3.0 * node.children()[0].size().width);
    assert!(node.children()[1].size().height != 3.0 * node.children()[0].size().height);
}
