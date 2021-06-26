use derive_more::Display;

#[derive(Display, Debug)]
pub enum DiError {
    ParseUuidError(uuid::Error),
    DatabaseError(sqlx::error::Error)
}

/* impl std::error::Error for DiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            &Self::ParseUuidError(e) => e.source(),
            &Self::DatabaseError(e) => e.as_database_error(),
        }
    }

} */
