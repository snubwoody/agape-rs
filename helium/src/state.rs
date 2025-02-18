use std::{cell::{Ref, RefCell}, sync::Arc};

#[derive(Debug)]
pub struct State<T>{
	value:Arc<RefCell<T>>
}

impl<T> State<T>{
	pub fn new(value:T) -> Self{
		Self{
			value:Arc::new(RefCell::new(value))
		}
	}

	/// Get a reference to the stateful value
	pub fn get(&self) -> Arc<RefCell<T>>{
		self.value.clone()
	}

	pub fn set(&self,value:T){
		self.value.replace(value);
	}
}

#[cfg(test)]
mod tests{
	use std::sync::{Mutex, RwLock};
	use crate::widgets::Text;
	use super::*;

	#[test]
	fn state(){
		println!("Hi");

		let count = State::new(5);
		let text = Text::new(&format!("{}",*count.get().borrow()));
		count.set(13);
	}
}