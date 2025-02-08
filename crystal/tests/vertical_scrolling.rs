use crystal::{AxisAlignment, BoxSizing, EmptyLayout, LayoutSolver, Size, VerticalLayout};

#[test]
fn scrolling(){
	let mut child = EmptyLayout::new();
	child.intrinsic_size.height = BoxSizing::Fixed(200.0);
	let children = vec![
		child.clone(),
		child.clone(),
		child.clone(),
		child.clone(),
		child.clone(),
		child
	];

	let scroll_offset = 100.0;
	let mut root = VerticalLayout::new();
	root.intrinsic_size.height = BoxSizing::Fixed(200.0);
	root.scroll(scroll_offset);
	root.add_children(children);

	let window = Size::unit(400.0);

	LayoutSolver::solve(&mut root, window);
	assert_eq!(root.children[0].position().y,scroll_offset);
}

#[test]
fn dont_scroll_if_no_overflow(){
	let children = vec![
		EmptyLayout::new(),
		EmptyLayout::new(),
		EmptyLayout::new(),
		EmptyLayout::new(),
		EmptyLayout::new(),
	];

	let scroll_offset = 100.0;
	let mut layout = VerticalLayout::new();
	layout.add_children(children);
	layout.scroll(12.0);

	let window = Size::new(400.0, 400.0);

	LayoutSolver::solve(&mut layout, window);
	assert_eq!(layout.children[0].position().y,0.0);
	
	layout.main_axis_alignment = AxisAlignment::Center;
	LayoutSolver::solve(&mut layout, window);
	assert_eq!(layout.children[0].position().y,0.0);
	
	layout.main_axis_alignment = AxisAlignment::End;
	LayoutSolver::solve(&mut layout, window);
	assert_eq!(layout.children[0].position().y,0.0);
}