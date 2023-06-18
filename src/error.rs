#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("login error")]
    LoginError,

    #[error("cookie IO error: {0}")]
    CookieIOError(Box<dyn std::error::Error + Sync + Send>),

    #[error("cookie store error: {0}")]
    CookieStoreError(#[from] cookie_store::CookieError),

    #[error("url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("unable to parse thread: {0}")]
    ThreadParsingError(String),

    #[error("unable to parse post: {0}")]
    PostParsingError(String),
}
