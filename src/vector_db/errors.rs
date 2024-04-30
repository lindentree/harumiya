use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct EmbeddingError;

impl std::error::Error for EmbeddingError {}
impl Display for EmbeddingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Embedding error")
    }
}

impl From<anyhow::Error> for EmbeddingError {
    fn from(_: anyhow::Error) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct NotAvailableError;

impl std::error::Error for NotAvailableError {}
impl std::fmt::Display for NotAvailableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File 'not available' error")
    }
}
