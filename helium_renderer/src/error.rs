use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    /// A resource was not found at it's expected location.  
    #[error("Resource not found:{0}")]
    ResourceNotFound(String),
}

impl Error {
	pub fn resource_not_found(message:&str) -> Self{
		Self::ResourceNotFound(message.to_string())
	}
}