use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("authorization error")]
    Authorization,

    #[error("validation error: {model}.{field}: {message}")]
    Validation {
        model: String,
        field: String,
        message: String,
    },

    #[error("query error: {0}")]
    QueryError(#[from] QueryError),

    #[error("misc error: {0}")]
    Misc(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("record not found")]
    NotFound,

    #[error("misc query error: {0}")]
    Misc(Box<dyn Error>),
}

impl From<diesel::result::Error> for DomainError {
    fn from(value: diesel::result::Error) -> Self {
        DomainError::QueryError(value.into())
    }
}

impl From<diesel::result::Error> for QueryError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::NotFound => Self::NotFound,
            err => Self::Misc(err.into()),
        }
    }
}
