use reqwest::StatusCode;
use thiserror::Error;
use Error::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // http error
    #[error("request url:{url} error, statusCode:{status_code}, message:{message}")]
    RestError {
        url: String,
        status_code: StatusCode,
        message: String,
    },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    Serialize(#[from] serde_json::Error),
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),
    #[error("venue not set")]
    VenueNotSet(),
}

// new http rest error
pub fn http_error(url: String, status_code: StatusCode, message: String) -> Error {
    RestError {
        url,
        status_code,
        message,
    }
}
