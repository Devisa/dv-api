use std::fmt;

pub struct PopError;

#[derive(Debug)]
pub enum WorkerExecError {
    Retryable,
    NotRetryable,
}

impl fmt::Debug for PopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "PopError".fmt(f)
    }
}
impl fmt::Display for PopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Popping from an empty queue".fmt(f)
    }
}
impl std::error::Error for PopError {
    fn description(&self) -> &str {
        "Popping from an empty queue"
    }
}

