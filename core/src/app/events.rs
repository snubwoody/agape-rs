/*  TODO stick thinking of a way to implement events
	I need to be able to pass variables down to the functions
	and I also need to have access to the widget itself, which isn't 
	too hard but I also need to be able to change properties like
	size which live on the widget body so I don't know if that'll
	even work because the body is built after the widget.
*/
pub enum EventFunction {
	OnClick(Box<dyn Fn()>),
	OnHover(Box<dyn Fn()>),
}