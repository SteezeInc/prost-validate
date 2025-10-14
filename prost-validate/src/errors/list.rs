use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("要素数は {0} 以上でなければなりません")]
    MinItems(usize),
    #[error("要素数は {0} 以下でなければなりません")]
    MaxItems(usize),
    #[error("要素が重複してはいけません")]
    Unique,
    #[error("{0}")]
    Item(Box<crate::Error>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::List(value)
    }
}
