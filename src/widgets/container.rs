use crate::widgets::Widget;



pub struct Container<'a>{
	padding:i32, 
	child:&'a dyn Widget
}
