use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // Third party errors
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("Event loop error: {0}")]
    EventLoopError(#[from] winit::error::EventLoopError),
}
