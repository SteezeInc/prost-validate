use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("要素数は {0} でなければなりません")]
    MinPairs(usize),
    #[error("要素数は {0} でなければなりません")]
    MaxPairs(usize),
    #[error("キー: {0}")]
    Keys(Box<crate::Error>),
    #[error("値: {0}")]
    Values(Box<crate::Error>),
    #[error("疎な値を含めてはいけません")]
    NoSparse,
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Map(value)
    }
}
