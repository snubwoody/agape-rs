use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
	/// A resource was not found at it's expected location.  
	#[error("{0} was not found")]
	NotFound(String)
}

