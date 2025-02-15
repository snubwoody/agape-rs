use std::{cell::{Ref, RefCell}, sync::Arc};

#[derive(Debug)]
pub struct State<T>{
	value:Arc<RefCell<T>>
}

impl<T> State<T>{
	pub fn new(value:T) -> Self{
		// TODO maybe i should make a custom type to prevent misuse of the inner refcell
		Self{
			value:Arc::new(RefCell::new(value))
		}
	}

	/// Get a reference to the stateful value
	pub fn get(&self) -> Arc<RefCell<T>>{
		self.value.clone()
	}

	pub fn set(&self,value:T){
		match self.value.try_borrow_mut() {
			Ok(mut v) => *v = value,
			Err(err) => println!("{err}")
		}
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
		state.set(20);
		dbg!(&a);
		dbg!(&k);
		dbg!(&v);
	}
}