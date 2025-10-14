#[macro_export]
macro_rules! make_error {
    ($name:ident, $typ:ident, $enum_value:ident) => {
        pub mod $name {
            use thiserror::Error;

            #[derive(Debug, Clone, Error)]
            pub enum Error {
                #[error("値は {0} と一致しなければなりません")]
                Const($typ),
                #[error("値は {0} 未満でなければなりません")]
                Lt($typ),
                #[error("値は {0} 以下でなければなりません")]
                Lte($typ),
                #[error("値は {0} より大きくなければなりません")]
                Gt($typ),
                #[error("値は {0:?} 以上でなければなりません")]
                Gte($typ),
                #[error("値は {0}{1}, {2}{3} の範囲内でなければなりません")]
                InRange(String, $typ, $typ, String),
                #[error("値は {0}{1}, {2}{3} の範囲内にあってはなりません")]
                NotInRange(String, $typ, $typ, String),
                #[error("値は {0:?} のいずれかでなければなりません")]
                In(Vec<$typ>),
                #[error("値は {0:?} のいずれかであってはなりません")]
                NotIn(Vec<$typ>),
            }

            impl Error {
                pub fn in_range(
                    start_inclusive: bool,
                    start: $typ,
                    end: $typ,
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
                    start: $typ,
                    end: $typ,
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
            impl From<Error> for $crate::errors::Error {
                fn from(value: Error) -> Self {
                    Self::$enum_value(value)
                }
            }
        }
    };
}
