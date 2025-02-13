use std::{cell::{Ref, RefCell}, sync::mpsc};

pub struct State{
	value:RefCell<i32>
}

impl State{
	pub fn new(value:i32) -> Self{
		Self{
			value:RefCell::new(value)
		}
	}

	pub fn get(&self) -> Ref<'_,i32>{
		self.value.borrow()
	}

	pub fn update(&self){
		*self.value.borrow_mut() += 1;
	}
}

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn state(){
		let count = State::new(0);
		let current = *count.get();
		
		assert_eq!(current,0);
		count.update();
		assert_eq!(current,1);
		
	}
}