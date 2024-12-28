use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Not found")]
    NotFoundError,
    #[error("Some error")]
    ServiceInternalError,
}
