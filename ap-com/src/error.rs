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
    InternalError,
}


impl std::error::Error for DiLibError {}
