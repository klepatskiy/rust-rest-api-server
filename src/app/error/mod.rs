pub mod service_error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFoundError,
    #[error("Some error")]
    SomeError,
}
