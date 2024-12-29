use crystal::{
	BlockLayout, BoxSizing, EmptyLayout, HorizontalLayout, Layout, LayoutSolver, Size, VerticalLayout
};

#[test]
fn test_horizontal_layout(){
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
	
	LayoutSolver::solve(&mut root,window);
	
	assert_eq!(
		root.size(),
		Size::new(900.0, 350.0)
	);

	assert_eq!(
		root.children()[0].size(),
		Size::new(400.0, 200.0)
	);

	assert_eq!(
		root.children()[1].size(),
		Size::new(500.0, 350.0)
	);
}

#[test]
fn test_flex_sizing(){
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
	
	LayoutSolver::solve(&mut root,window);
	let child_size = Size::new(400.0, 800.0);
	assert_eq!(root.size(),window);
	assert_eq!(root.children()[0].size(),child_size);
	assert_eq!(root.children()[1].size(),child_size);
}

#[test]
fn test_empty_inner_flex_size(){
	// Test that inner flex widgets have a width of zero when their parent is set
	// to shrink
	todo!()
}

#[test]
fn test_flex_with_shrink(){
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

	let mut child_2_size = window;
	child_2_size.width -= child_1_size.width;
	child_2_size.width -= spacing as f32;
	child_2_size -= (padding * 2) as f32;
	
	assert_eq!(
		root.size(),
		root_size
	);
	assert_eq!(
		root.children[0].size(),
		child_1_size
	);
	assert_eq!(
		root.children[1].size(),
		child_2_size
	);
}

#[test]
fn test_flex_with_fixed(){
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
	root.add_child(child_1);
	root.add_child(child_2);
	root.add_child(child_3);
	
	LayoutSolver::solve(&mut root, window);

	let mut space = window;
	space -= (padding * 2) as f32;
	space -= (spacing * 2) as f32;
	space.width -= 250.0;

	assert_eq!(
		root.children[1].size().width,
		1.0/5.0 * space.width
	);
	assert_eq!(
		root.children[2].size().width,
		4.0/5.0 * space.width
	);
	assert_eq!(
		root.children[1].size().height,
		window.height - (padding * 2) as f32
	);
}

#[test]
fn test_different_layouts(){
	let window = Size::new(800.0, 800.0);
	let padding = 20;
	let spacing = 54;

	let mut child = EmptyLayout::new();
	child.intrinsic_size.width = BoxSizing::Fixed(250.0);
	child.intrinsic_size.height = BoxSizing::Flex(1);
	
	let mut block_child = EmptyLayout::new();
	block_child.intrinsic_size.width = BoxSizing::Fixed(250.0);
	block_child.intrinsic_size.height = BoxSizing::Fixed(10.0);
	
	let mut block = BlockLayout::new(Box::new(block_child));
	block.padding = padding;
	
	let mut vertical_child = EmptyLayout::new();
	vertical_child.intrinsic_size.width = BoxSizing::Fixed(250.0);
	vertical_child.intrinsic_size.height = BoxSizing::Fixed(100.0);

	let mut vertical_child_2 = EmptyLayout::new();
	vertical_child_2.intrinsic_size.width = BoxSizing::Fixed(500.0);
	vertical_child_2.intrinsic_size.height = BoxSizing::Fixed(100.0);
	
	let mut vertical = VerticalLayout::new();
	vertical.add_child(vertical_child);
	vertical.add_child(vertical_child_2);
	vertical.spacing = spacing;
	vertical.padding = padding;
	
	let mut horizontal_child = EmptyLayout::new();
	horizontal_child.intrinsic_size.width = BoxSizing::Fixed(250.0);
	horizontal_child.intrinsic_size.height = BoxSizing::Fixed(100.0);
	
	let mut horizontal_child_2 = EmptyLayout::new();
	horizontal_child_2.intrinsic_size.width = BoxSizing::Fixed(20.0);
	horizontal_child_2.intrinsic_size.height = BoxSizing::Flex(1);
	
	let mut horizontal = HorizontalLayout::new();
	horizontal.add_child(horizontal_child);
	horizontal.add_child(horizontal_child_2);
	horizontal.padding = padding;
	horizontal.spacing = spacing;
	
	let mut root = HorizontalLayout::new();
	root.spacing = padding;
	root.padding = spacing;
	root.add_child(child);
	root.add_child(block);
	root.add_child(vertical);
	root.add_child(horizontal);

	LayoutSolver::solve(&mut root, window);

	let block_size = Size::new(250.0, 10.0) + padding as f32 * 2.0;
	let mut horizontal_size = Size::new(270.0, 100.0) + padding as f32 * 2.0;
	horizontal_size.width += spacing as f32;
	
	let mut vertical_size = Size::new(500.0, 200.0) + padding as f32 * 2.0;
	vertical_size.height += spacing as f32;

	let horizontal_child_2_size = Size::new(20.0, 100.0);
	
	let mut root_size = Size::default();
	root_size.width += 250.0; // First child
	root_size.width += block_size.width; 
	root_size.width += vertical_size.width; 
	root_size.width += horizontal_size.width;
	root_size.width += spacing as f32 * 3.0;
	root_size.width += padding as f32 * 2.0;
	root_size.height += padding as f32 * 2.0;
	root_size.height += vertical_size.height;
	
	assert_eq!(
		root.children[1].size(),
		block_size
	); // Block child
	assert_eq!(
		root.children[1].size(),
		block_size
	);
	assert_eq!(
		root.children[2].size(),
		vertical_size
	);
	assert_eq!(
		root.children[3].size(),
		horizontal_size
	);
	assert_eq!(
		root.children[3].children()[1].size(),
		horizontal_child_2_size
	);
	assert_eq!(
		root.children[0].size().height,
		root.size().height - padding as f32 * 2.0
	); // First child
	assert_eq!(
		root.size(),
		root_size
	);

}

// TODO test flex grow inside flex shrink
#[test]
fn test_flex_factor(){
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

	LayoutSolver::solve(&mut node,window);

	let flex_1_width = 1.0/4.0 * window.width;
	// The two children should both be half the size
	assert_eq!(
		node.children()[0].size().width,
		flex_1_width
	);
	assert_eq!(node.children()[0].size().height,400.0);
	assert_eq!(
		node.children()[0].size().height,
		node.children()[1].size().height,
	);
	assert!(
		node.children()[1].size().width == 3.0 * node.children()[0].size().width
	);
	assert!(
		node.children()[1].size().height != 3.0 * node.children()[0].size().height
	);
}