use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported image format, only png, jpeg and webp are supported")]
    UnsupportedImageFormat,

    // Third party errors
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    EventLoopError(#[from] winit::error::EventLoopError),
}
