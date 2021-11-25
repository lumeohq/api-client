use std::fmt;

use reqwest::{Method, Response, StatusCode};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use thiserror::Error;

use crate::Result;

#[derive(EnumString, Error, Debug)]
#[strum(serialize_all = "kebab-case")]
pub enum ApiError {
    #[error("The gateway this access token belonged to has been deleted (`gateway-deleted`)")]
    GatewayDeleted,
    #[error("User credentials are invalid (`invalid-credentials`)")]
    InvalidCredentials,
    #[doc(hidden)]
    #[strum(disabled)]
    #[error("{message} (`{code}`)")]
    Other { code: String, message: String },
}

// Response from server
#[derive(Debug, Deserialize, Serialize)]
struct ApiServerResponse {
    code: String,
    message: String,
}

impl<'de> Deserialize<'de> for ApiError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let resp = ApiServerResponse::deserialize(deserializer)?;
        Ok(match resp.code.parse::<ApiError>() {
            Ok(api_err) => api_err,
            Err(_) => ApiError::Other { code: resp.code, message: resp.message },
        })
    }
}

/// Checks if response status isn't a success,
/// and then tries to extract json error description from it
///
/// # Returns
///
/// Result with untouched [`Response`] or error if the status isn't a success
pub(crate) async fn verify_response(
    response: Result<Response, reqwest::Error>,
    method: Method,
    path: &str,
) -> Result<Response> {
    let method_cp = method.clone();
    let response = response.map_err(|e| {
        let status = e.status();
        Error::Reqwest(e, ErrorDetails::new(method, path, status))
    })?;

    if !response.status().is_success() {
        let cp = method_cp.clone();
        let status = Some(response.status());
        return Err(response.json::<ApiError>().await.map_or_else(
            |e| Error::ErrorDeserialization(e, ErrorDetails::new(method_cp.clone(), path, status)),
            |e| Error::Api(e, ErrorDetails::new(cp, path, None)),
        ));
    }

    Ok(response)
}

#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub method: Method,
    pub path: String,
    pub status: Option<StatusCode>,
}

impl ErrorDetails {
    fn new(method: Method, path: &str, status: Option<StatusCode>) -> Self {
        Self { method, path: path.to_owned(), status }
    }
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            Some(status_code) => write!(
                f,
                "{} request to `{}` failed with status code {}",
                self.method, self.path, status_code,
            ),
            None => {
                write!(f, "{} request to `{}` failed", self.method, self.path)
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{1}: {0}")]
    Url(#[source] url::ParseError, ErrorDetails),
    #[error("{1}: {0}")]
    Query(#[source] serde_urlencoded::ser::Error, ErrorDetails),
    #[error("{1}: {0}")]
    Reqwest(#[source] reqwest::Error, ErrorDetails),
    #[error("{1}: {0}")]
    Api(#[source] ApiError, ErrorDetails),
    #[error("{1}: {0}")]
    ErrorDeserialization(#[source] reqwest::Error, ErrorDetails),
    #[error("Application id is missing")]
    ApplicationIdMissing,
    #[error("Gateway id is missing")]
    GatewayIdMissing,
}

pub(crate) trait ResultExt<T> {
    fn http_context(self, method: Method, path: &str) -> Result<T>;
}

impl<T> ResultExt<T> for Result<T, url::ParseError> {
    fn http_context(self, method: Method, path: &str) -> Result<T> {
        self.map_err(|e| Error::Url(e, ErrorDetails::new(method, path, None)))
    }
}

impl<T> ResultExt<T> for Result<T, serde_urlencoded::ser::Error> {
    fn http_context(self, method: Method, path: &str) -> Result<T> {
        self.map_err(|e| Error::Query(e, ErrorDetails::new(method, path, None)))
    }
}

impl<T> ResultExt<T> for Result<T, reqwest::Error> {
    fn http_context(self, method: Method, path: &str) -> Result<T> {
        self.map_err(|e| {
            let status = e.status();
            Error::Reqwest(e, ErrorDetails::new(method, path, status))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gateway_deleted() {
        let resp =
            ApiServerResponse { code: "gateway-deleted".to_owned(), message: Default::default() };

        let error: ApiError = serde_json::from_str(&serde_json::to_string(&resp).unwrap()).unwrap();
        assert!(matches!(error, ApiError::GatewayDeleted));
    }

    #[test]
    fn invalid_credentials() {
        let resp = ApiServerResponse {
            code: "invalid-credentials".to_owned(),
            message: Default::default(),
        };

        let error: ApiError = serde_json::from_str(&serde_json::to_string(&resp).unwrap()).unwrap();
        assert!(matches!(error, ApiError::InvalidCredentials));
    }
}
