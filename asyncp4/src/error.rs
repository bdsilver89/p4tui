use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("`{0}`")]
    Generic(String),

    #[error("io error:{0}")]
    Io(#[from] std::io::Error),

    #[error("uft8 error:{0}")]
    Utf8Conversion(#[from] FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        Self::Generic(format!("poison error: {error}"))
    }
}

impl<T> From<crossbeam_channel::SendError<T>> for Error {
    fn from(error: crossbeam_channel::SendError<T>) -> Self {
        Self::Generic(format!("send error: {error}"))
    }
}
