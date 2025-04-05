use thiserror::Error;

pub type Result<T> = std::result::Result<T,Error>;

#[derive(Debug, Error,)]
pub enum Error {
    /// A resource was not found at it's expected location.  
    #[error("Resource not found:{0}")]
    ResourceNotFound(String),
	#[error(transparent)]
	EventLoopError(#[from] winit::error::EventLoopError),
	#[error(transparent)]
	OsError(#[from] winit::error::OsError),
}

impl Error {
    pub fn resource_not_found(message: &str) -> Self {
        Self::ResourceNotFound(message.to_string())
    }
}
