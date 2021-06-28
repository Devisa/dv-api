use uuid::{self, Uuid};
use std::{error, fmt};

#[derive(Debug)]
pub enum DdbError {
    FailedToAcquireFromPool,
    NotFound,
    ParseUuidError(uuid::Error),
    ConnectionFailed(sqlx::Error),
    DatabaseError(sqlx::Error),
}

pub type DdbResult<T> = Result<T, DdbError>;

impl fmt::Display for DdbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseUuidError(e) => f.write_fmt(format_args!("Parse UUID error: {}", e)),
            Self::ConnectionFailed(e) => f.write_fmt(format_args!("Conn failed: {}", e)),
            Self::DatabaseError(e) => f.write_fmt(format_args!("DB err: {}", e)),

            _ => f.write_str("Error")
        }
    }
}

impl error::Error for DdbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseUuidError(e) => e.source(),
            Self::ConnectionFailed(e) => None,
            Self::DatabaseError(e) => None,
            _ => None,
        }
    }
}

impl From<uuid::Error> for DdbError {
    fn from(e: uuid::Error) -> Self {
        Self::ParseUuidError(e)
    }
}
