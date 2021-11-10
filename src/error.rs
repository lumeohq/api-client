use reqwest::{Method, Response};
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
    let response = response.map_err(|e| Error::Reqwest(e, ErrorDetails::new(method, path)))?;

    if !response.status().is_success() {
        let cp = method_cp.clone();
        return Err(response.json::<ApiError>().await.map_or_else(
            |e| Error::ErrorDeserialization(e, ErrorDetails::new(method_cp.clone(), path)),
            |e| Error::Api(e, ErrorDetails::new(cp, path)),
        ));
    }

    Ok(response)
}

#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub method: Method,
    pub path: String,
}

impl ErrorDetails {
    fn new(method: Method, path: &str) -> Self {
        Self { method, path: path.to_owned() }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{} request to `{}` failed: {0}", .1.method, .1.path)]
    Url(#[source] url::ParseError, ErrorDetails),
    #[error("{} request to `{}` failed: {0}", .1.method, .1.path)]
    Query(#[source] serde_urlencoded::ser::Error, ErrorDetails),
    #[error("{} request to `{}` failed: {0}", .1.method, .1.path)]
    Reqwest(#[source] reqwest::Error, ErrorDetails),
    #[error("{} request to `{}` failed: {0}", .1.method, .1.path)]
    Api(#[source] ApiError, ErrorDetails),
    #[error("{} request to `{}` failed: {0}", .1.method, .1.path)]
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
        self.map_err(|e| Error::Url(e, ErrorDetails::new(method, path)))
    }
}

impl<T> ResultExt<T> for Result<T, serde_urlencoded::ser::Error> {
    fn http_context(self, method: Method, path: &str) -> Result<T> {
        self.map_err(|e| Error::Query(e, ErrorDetails::new(method, path)))
    }
}

impl<T> ResultExt<T> for Result<T, reqwest::Error> {
    fn http_context(self, method: Method, path: &str) -> Result<T> {
        self.map_err(|e| Error::Reqwest(e, ErrorDetails::new(method, path)))
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
