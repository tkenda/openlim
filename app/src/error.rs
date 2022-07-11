use thiserror::Error;

use crate::{ASTMError, InstError};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("ASTM error: {0}")]
    ASTM(ASTMError),
    #[error("Instruments error: {0}")]
    Inst(InstError),
}

impl From<ASTMError> for Error {
    fn from(error: ASTMError) -> Self {
        Self::ASTM(error)
    }
}

impl From<InstError> for Error {
    fn from(error: InstError) -> Self {
        Self::Inst(error)
    }
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        error.to_string()
    }
}