use core::fmt;
use std::error::Error;
// CUSTOM ERROR STRUCTS + ENUMS
// -----------------------------------------------

/// A comprehensive set of errors that might occur in the Kalshi module.
///
/// This enum encompasses various types of errors, including HTTP request errors,
/// user input errors, and internal errors. It provides a unified error type for
/// the entire Kalshi module.
///
#[derive(Debug)]
pub enum KalshiError {
    /// Errors that occur during HTTP requests. This includes connectivity issues,
    /// response serialization problems, and HTTP status errors.
    RequestError(RequestError),
    /// Errors caused by incorrect or invalid user input.
    UserInputError(String),
    /// Errors representing unexpected internal issues or situations that are not supposed to happen.
    InternalError(String),
    // TODO: add error type specifically for joining threads together.
}

impl fmt::Display for KalshiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KalshiError::RequestError(e) => write!(f, "HTTP Error: {}", e),
            KalshiError::UserInputError(e) => write!(f, "User Input Error: {}", e),
            KalshiError::InternalError(e) => write!(f, "INTERNAL ERROR, PLEASE EMAIL DEVELOPER OR MAKE A NEW ISSUE ON THE CRATE'S REPOSITORY: https://github.com/dpeachpeach/kalshi-rust. Specific Error: {}", e)
        }
    }
}

impl Error for KalshiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            KalshiError::RequestError(e) => Some(e),
            KalshiError::UserInputError(_) => None,
            KalshiError::InternalError(_) => None,
        }
    }
}

impl From<reqwest::Error> for KalshiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_decode() {
            KalshiError::RequestError(RequestError::SerializationError(err))
        } else if err.is_status() {
            if let Some(status) = err.status() {
                if status.is_client_error() {
                    KalshiError::RequestError(RequestError::ClientError(err))
                } else if status.is_server_error() {
                    KalshiError::RequestError(RequestError::ServerError(err))
                } else {
                    KalshiError::InternalError(
                        "Theoretically Impossible Error. Internal code 1".to_string(),
                    )
                }
            } else {
                KalshiError::RequestError(RequestError::ServerError(err))
            }
        } else if err.is_body() || err.is_timeout() {
            KalshiError::RequestError(RequestError::ServerError(err))
        } else {
            KalshiError::InternalError(
                "Theoretically Impossible Error. Internal code 2".to_string(),
            )
        }
    }
}

/// Specific kinds of HTTP request errors encountered in the Kalshi module.
///
/// This enum categorizes errors related to HTTP requests, including serialization errors, client-side errors,
/// and server-side errors.
///
#[derive(Debug)]
pub enum RequestError {
    /// Errors occurring during serialization or deserialization of request or response data.
    SerializationError(reqwest::Error),
    /// Errors representing client-side request issues, such as bad requests or unauthorized access.
    ClientError(reqwest::Error),
    /// Errors indicating server-side issues, like internal server errors or service unavailability.
    ServerError(reqwest::Error),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::SerializationError(e) => write!(f, "Serialization Error. You connected successfully but either: Your inputs to a request were incorrect or the exchange is closed! {}", e),
            RequestError::ClientError(e) => {
                if let Some(status) = e.status() {
                    write!(f, "Client Request Error, Status code: {}", status)
                } else {
                    write!(f, "Client Request Error: {}", e)
                }
            },
            RequestError::ServerError(e) => {
                if let Some(status) = e.status() {
                    write!(f, "Server Request Error: Status code: {}", status)
                } else {
                    write!(f, "Server Request Error: {}", e)
                }
            },
        }
    }
}

impl Error for RequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RequestError::ClientError(e) => Some(e),
            RequestError::ServerError(e) => Some(e),
            RequestError::SerializationError(e) => Some(e),
        }
    }
}
