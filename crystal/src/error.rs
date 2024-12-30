use thiserror::Error;

#[derive(Debug,Clone,Copy,Error,PartialEq,Eq)]
pub enum LayoutError {
	#[error("Widget(id:{child_id}) is out of it's parent's (id:{parent_id}) bounds")]
    OutOfBounds{
		parent_id:&'static str, 
		child_id:&'static str, 
	},
	#[error("Widget's (id:{0}) children have overflown")]
	Overflow(&'static str),
}