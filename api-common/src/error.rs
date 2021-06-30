use derive_more::Display;

pub type DiLibResult<T> = Result<T, DiLibError>;

#[derive(Display, Debug)]
#[display(fmt = "api_common error")]
pub enum DiLibError {
    ParseUuidError(uuid::Error),
    SerializeError(String),
    DeserializeError(String),
    DatabaseError(sqlx::error::Error),
    IoError(std::io::Error),
}

/* impl std::error::Error for DiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            &Self::ParseUuidError(e) => e.source(),
            &Self::DatabaseError(e) => e.as_database_error(),
        }
    }

} */
