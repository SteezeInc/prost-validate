use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値が必須です")]
    Required,
    #[error("type_url は {0:?} のいずれかでなければなりません")]
    In(Vec<String>),
    #[error("type_url は {0:?} のいずれかであってはなりません")]
    NotIn(Vec<String>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Any(value)
    }
}
