use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値は {0:?} と一致しなければなりません")]
    Const(time::Duration),
    #[error("値は {0:?} 未満でなければなりません")]
    Lt(time::Duration),
    #[error("値は {0:?} 以下でなければなりません")]
    Lte(time::Duration),
    #[error("値は {0:?} より大きくなければなりません")]
    Gt(time::Duration),
    #[error("値は {0:?} 以上でなければなりません")]
    Gte(time::Duration),
    #[error("値は {0}{1:?}、{2:?}{3} の範囲内でなければなりません")]
    InRange(String, time::Duration, time::Duration, String),
    #[error("値は {0}{1:?}、{2:?}{3} の範囲内にあってはなりません")]
    NotInRange(String, time::Duration, time::Duration, String),
    #[error("値は {0:?} のいずれかでなければなりません")]
    In(Vec<time::Duration>),
    #[error("値は {0:?} のいずれかであってはなりません")]
    NotIn(Vec<time::Duration>),
}

impl Error {
    pub fn in_range(
        start_inclusive: bool,
        start: time::Duration,
        end: time::Duration,
        end_inclusive: bool,
    ) -> Self {
        Self::InRange(
            if start_inclusive { "[" } else { "(" }.to_string(),
            start,
            end,
            if end_inclusive { "]" } else { ")" }.to_string(),
        )
    }
    pub fn not_in_range(
        start_inclusive: bool,
        start: time::Duration,
        end: time::Duration,
        end_inclusive: bool,
    ) -> Self {
        Self::NotInRange(
            if start_inclusive { "[" } else { "(" }.to_string(),
            start,
            end,
            if end_inclusive { "]" } else { ")" }.to_string(),
        )
    }
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Duration(value)
    }
}
