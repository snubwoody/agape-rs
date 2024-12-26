//! Just testing for future implementation

/// A signal is reactive state
pub struct Signal<T>{
	value:T
}

impl<'s,T> Signal<T> {
	/// Create a new signal
	pub fn create(value:T) -> Self {
		Self { value }
	}

	pub fn get(&self) -> &T{
		&self.value
	}

	/// Updates the signal value, while passing the old value as a parameter
	pub fn update(&mut self,mut func:impl FnMut(&T) -> T) {
		self.value = func(&self.value)
	}

	/// Set the signal value
	pub fn set(&mut self,value:T){
		self.value = value;
	}
}