use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値は {0:?} と一致しなければなりません")]
    Const(Vec<u8>),
    #[error("バイト数は {0} でなければなりません")]
    Len(usize),
    #[error("バイト数は {0} 以上でなければなりません")]
    MinLen(usize),
    #[error("バイト数は {0} 以下でなければなりません")]
    MaxLen(usize),
    #[error("パターン {0} に一致しなければなりません")]
    Pattern(String),
    #[error("接頭辞 {0:?} を持つ必要があります")]
    Prefix(Vec<u8>),
    #[error("接尾辞 {0:?} を持つ必要があります")]
    Suffix(Vec<u8>),
    #[error("{0:?} を含む必要があります")]
    Contains(Vec<u8>),
    #[error("{0:?} のいずれかでなければなりません")]
    In(Vec<Vec<u8>>),
    #[error("{0:?} のいずれかであってはなりません")]
    NotIn(Vec<Vec<u8>>),
    #[error("有効な IP アドレスでなければなりません")]
    Ip,
    #[error("有効な IPv4 アドレスでなければなりません")]
    Ipv4,
    #[error("有効な IPv6 アドレスでなければなりません")]
    Ipv6,
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Bytes(value)
    }
}
