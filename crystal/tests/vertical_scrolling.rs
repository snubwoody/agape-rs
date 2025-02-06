use crystal::{EmptyLayout, LayoutSolver, Size, VerticalLayout};


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
	layout.add_children(children);

	let window = Size::new(400.0, 400.0);

	LayoutSolver::solve(&mut layout, window);

	assert_eq!(layout.children[0].position().y,scroll_offset);
}