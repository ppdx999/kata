use thiserror::Error as ThisError;
use crate::schema::{SchemaError, ValidationError};

#[derive(ThisError, Debug, PartialEq)]
pub enum Error {
    #[error(transparent)]
    Schema(#[from] SchemaError),

    #[error(transparent)]
    Validation(#[from] ValidationError),
}
