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
	use super::*;

	#[test]
	fn state(){
		let state = State::new(10);
		let k = state.get();
		let v = state.get();
		let a = k.borrow();
		
		dbg!(&k);
		dbg!(&v);

		state.set(20); // Panics here
		
		dbg!(&a);
		dbg!(&k);
		dbg!(&v);
	}
}