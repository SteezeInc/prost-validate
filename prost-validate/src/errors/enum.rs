use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値は {0:?} と一致しなければなりません")]
    Const(i32),
    #[error("定義済みの値のみ使用できます")]
    DefinedOnly,
    #[error("値は {0:?} のいずれかでなければなりません")]
    In(Vec<i32>),
    #[error("値は {0:?} のいずれかであってはなりません")]
    NotIn(Vec<i32>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Enum(value)
    }
}
