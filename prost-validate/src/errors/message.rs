use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値が必須です")]
    Required,
    #[error("{0}")]
    Message(Box<crate::Error>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Message(value)
    }
}
