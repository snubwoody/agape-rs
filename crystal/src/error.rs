use thiserror::Error;

#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum OverflowAxis{
	MainAxis,
	CrossAxis
}

impl std::fmt::Display for OverflowAxis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{:?}", self)
	}
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum LayoutError {
    #[error("Widget(id:{child_id}) is out of it's parent's (id:{parent_id}) bounds")]
    OutOfBounds { parent_id: String, child_id: String },

	#[error("Widget(id:{id})'s children have overflown in the {axis}")]
    Overflow{
		id: String,
		axis: OverflowAxis,
	},
}

impl LayoutError {
	pub fn out_of_bound(parent_id:&str,child_id:&str) -> Self{
		Self::OutOfBounds { 
			parent_id: String::from(parent_id), 
			child_id: String::from(child_id) 
		}
	}

	pub fn overflow(id:&str,axis:OverflowAxis) -> Self{
		Self::Overflow { id: String::from(id), axis }
	}
}
