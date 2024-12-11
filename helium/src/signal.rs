//! Just testing for future implementation

/// A signal is reactive state
pub struct Signal<T>{
	value:T
}

impl<T> Signal<T> {
	/// Create a new signal
	fn create(value:T) -> Self {
		Self { value }
	}

	fn get(&self) -> &T{
		&self.value
	}

	/// Updates the signal value, while passing the old value as a parameter
	fn update(&mut self,mut func:impl FnMut(&T) -> T + 'static) {
		self.value = func(&self.value)
	}

	/// Set the signal value
	fn set(&mut self,value:T){
		self.value = value;
	}
}