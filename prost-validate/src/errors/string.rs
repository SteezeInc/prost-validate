use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("値は \"{0}\" と一致しなければなりません")]
    Const(String),
    #[error("文字数は {0} でなければなりません")]
    Len(usize),
    #[error("文字数は {0} 以上でなければなりません")]
    MinLen(usize),
    #[error("文字数は {0} 以下でなければなりません")]
    MaxLen(usize),
    #[error("バイト数は {0} でなければなりません")]
    LenBytes(usize),
    #[error("バイト数は {0} 以上でなければなりません")]
    MinLenBytes(usize),
    #[error("バイト数は {0} 以下でなければなりません")]
    MaxLenBytes(usize),
    #[error("\"{0}\" のパターンに一致しなければなりません")]
    Pattern(String),
    #[error("\"{0}\" を接頭辞に持つ必要があります")]
    Prefix(String),
    #[error("\"{0}\" を接尾辞に持つ必要があります")]
    Suffix(String),
    #[error("\"{0}\" を含む必要があります")]
    Contains(String),
    #[error("\"{0}\" を含んではいけません")]
    NotContains(String),
    #[error("値は {0:?} のいずれかでなければなりません")]
    In(Vec<String>),
    #[error("値は {0:?} のいずれかであってはなりません")]
    NotIn(Vec<String>),
    #[error("有効なメールアドレスでなければなりません")]
    Email,
    #[error("有効なホスト名でなければなりません")]
    Hostname,
    #[error("有効な IP アドレスでなければなりません")]
    Ip,
    #[error("有効な IPv4 アドレスでなければなりません")]
    Ipv4,
    #[error("有効な IPv6 アドレスでなければなりません")]
    Ipv6,
    #[error("有効な URI でなければなりません")]
    Uri,
    #[error("有効な URI 参照でなければなりません")]
    UriRef,
    #[error("有効なホスト名または IP アドレスでなければなりません")]
    Address,
    #[error("有効な UUID でなければなりません")]
    Uuid,
    #[error("有効な HTTP ヘッダー名でなければなりません")]
    HttpHeaderName,
    #[error("有効な HTTP ヘッダー値でなければなりません")]
    HttpHeaderValue,
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::String(value)
    }
}
