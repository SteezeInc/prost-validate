use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値は {0:?} と一致しなければなりません")]
    Const(time::OffsetDateTime),
    #[error("値は {0:?} 未満でなければなりません")]
    Lt(time::OffsetDateTime),
    #[error("値は {0:?} 以下でなければなりません")]
    Lte(time::OffsetDateTime),
    #[error("値は {0:?} より大きくなければなりません")]
    Gt(time::OffsetDateTime),
    #[error("値は {0:?} 以上でなければなりません")]
    Gte(time::OffsetDateTime),
    #[error("値は {0}{1:?}、{2:?}{3} の範囲内でなければなりません")]
    InRange(String, time::OffsetDateTime, time::OffsetDateTime, String),
    #[error("値は {0}{1:?}、{2:?}{3} の範囲内にあってはなりません")]
    NotInRange(String, time::OffsetDateTime, time::OffsetDateTime, String),
    #[error("現在時刻より前でなければなりません")]
    LtNow,
    #[error("現在より前、または今から {0} 以内でなければなりません")]
    LtNowWithin(time::Duration),
    #[error("現在時刻より後でなければなりません")]
    GtNow,
    #[error("現在より後、または今から {0} 以内でなければなりません")]
    GtNowWithin(time::Duration),
    #[error("現在時刻から {0:?} 以内でなければなりません")]
    Within(time::Duration),
}

impl Error {
    pub fn in_range(
        start_inclusive: bool,
        start: time::OffsetDateTime,
        end: time::OffsetDateTime,
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
        start: time::OffsetDateTime,
        end: time::OffsetDateTime,
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
        Self::Timestamp(value)
    }
}
