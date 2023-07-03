#[derive(Debug, thiserror::Error)]
pub enum RspError {
    // #[error("Executing DB query: {0}")]
    // PgError(#[from] tokio_postgres::Error),
    // #[error("Deadpool: {0}")]
    // PoolError(#[from] deadpool_postgres::PoolError),
    #[error("Not get user agent")]
    EmptyUserAgent,
    #[error("Unsupported version {0}")]
    VersionUnsupported(String),
    #[error("Unsupported method {0}")]
    MethodUnsupported(String),
    #[error("Header incomplete")]
    HeaderIncomplete,
    #[error("Header more when 1024")]
    HeaderToBig,
    #[error("Input output: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse utf8")]
    FromUtf8(#[from] std::str::Utf8Error),
    #[error("Method is empty")]
    EmptyMethod,
    #[error("Headers is empty")]
    EmptyHeader,
    #[error("Version is empty")]
    EmptyVersion,
    #[error("Request uri is empty")]
    EmptyRequestUri,
    #[error("Request line more when 3 chunks")]
    RequestLineToBig,
    #[error("Parse int")]
    ParseInt(#[from] std::num::ParseIntError),
    // #[error("Netc: {0}")]
    // FromNetc(#[from] netc::Error),
    #[error("Regex {0}")]
    FromRegex(#[from] regex::Error),
    // #[error("Reqwest: {0}")]
    // FromReqwest(#[from] reqwest::Error),
    #[error("Netc: {0}")]
    FromUreq(#[from] netc::Error),
}

impl From<RspError> for std::io::Error {
    fn from(err: RspError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}
