use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値は {0} と等しくなければなりません")]
    Const(bool),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Bool(value)
    }
}
