use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenF1Error {
    #[error("OpenF1 request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("OpenF1 returned HTTP {status}: {body}")]
    Http { status: u16, body: String },
    #[error("OpenF1 did not return a session")]
    NoSession,
}
