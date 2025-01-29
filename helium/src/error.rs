use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /* Third party wrappers */
    #[error(transparent)]
    EventLoopError(#[from] winit::error::EventLoopError),
}
