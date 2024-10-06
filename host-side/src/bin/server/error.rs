#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("websockets error")]
    TWSError(#[from] tokio_websockets::Error),
    #[error("std::io error")]
    StdIoError(#[from] std::io::Error),
    #[error("serde_json error")]
    SerdeJsonError(#[from] serde_json::Error),
}
