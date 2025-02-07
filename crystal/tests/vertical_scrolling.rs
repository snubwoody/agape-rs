use crystal::{AxisAlignment, EmptyLayout, LayoutSolver, Position, Size, VerticalLayout};


#[test]
fn scrolling(){
	let children = vec![
		EmptyLayout::new(),
		EmptyLayout::new(),
		EmptyLayout::new(),
		EmptyLayout::new(),
		EmptyLayout::new(),
	];

	let scroll_offset = 100.0;
	let mut layout = VerticalLayout::new();
	layout.scroll_offset = scroll_offset;
	layout.add_children(children);

	let window = Size::new(400.0, 400.0);

	// Test scrolling for every main_axis_alignment
	LayoutSolver::solve(&mut layout, window);
	assert_eq!(layout.children[0].position().y,scroll_offset);
	
	layout.main_axis_alignment = AxisAlignment::Center;
	LayoutSolver::solve(&mut layout, window);
	assert_eq!(layout.children[0].position().y,scroll_offset);
	
	layout.main_axis_alignment = AxisAlignment::End;
	LayoutSolver::solve(&mut layout, window);
	assert_eq!(layout.children[0].position().y,scroll_offset);

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