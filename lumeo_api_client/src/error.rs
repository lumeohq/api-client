use std::fmt;

use reqwest::{Method, Response, StatusCode};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
use thiserror::Error;

const RESOURCE_KEY: &str = "resource";

use crate::Result;

#[derive(AsRefStr, Error, Debug)]
#[strum(serialize_all = "kebab-case")]
pub enum ApiError {
    #[error("The gateway this access token belonged to has been deleted (`gateway-deleted`)")]
    GatewayDeleted,
    #[error("User credentials are invalid (`invalid-credentials`)")]
    InvalidCredentials,
    #[error("Resource not found (`resource-not-found`), resource: {0}")]
    ResourceNotFound(#[source] ResourceNotFound),
    #[doc(hidden)]
    #[strum(disabled)]
    #[error("{message} (`{code}`)")]
    Other { code: String, message: String },
}

#[derive(EnumString, Debug, Error)]
pub enum ResourceNotFound {
    #[error("Deployment")]
    #[strum(serialize = "deployment")]
    DeploymentNotFound,
    #[doc(hidden)]
    #[strum(disabled)]
    #[error("Other({0})")]
    Other(String),
}

impl ResourceNotFound {
    fn from_context(context: serde_json::Value) -> Option<Self> {
        let resource = context.as_object()?.get(RESOURCE_KEY)?.as_str()?;

        let resource_not_found = resource
            .parse::<ResourceNotFound>()
            .unwrap_or_else(|_| ResourceNotFound::Other(resource.to_owned()));
        Some(resource_not_found)
    }
}

impl Default for ResourceNotFound {
    fn default() -> Self {
        Self::Other("".to_owned())
    }
}

// Response from server
#[derive(Debug, Deserialize, Serialize, Default)]
struct ApiServerResponse {
    code: String,
    message: String,
    context: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for ApiError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ApiServerResponse { code, message, context } =
            ApiServerResponse::deserialize(deserializer)?;

        Ok(if code == ApiError::GatewayDeleted.as_ref() {
            ApiError::GatewayDeleted
        } else if code == ApiError::InvalidCredentials.as_ref() {
            ApiError::InvalidCredentials
        } else if code == ApiError::ResourceNotFound(Default::default()).as_ref() {
            context
                .and_then(ResourceNotFound::from_context)
                .map_or_else(|| ApiError::Other { code, message }, ApiError::ResourceNotFound)
        } else {
            ApiError::Other { code, message }
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
    let response = response.map_err(|e| {
        let status = e.status();
        Error::Reqwest(e, ErrorDetails::new(method.clone(), path, status))
    })?;

    if !response.status().is_success() {
        let details = ErrorDetails::new(method, path, Some(response.status()));
        let body = match response.bytes().await {
            Ok(b) => b,
            Err(e) => return Err(Error::Reqwest(e, details)),
        };

        if body.is_empty() {
            return Err(Error::ApiEmptyResponse(details));
        }

        match serde_json::from_slice(&body) {
            Ok(api_error) => return Err(Error::Api(api_error, details)),
            Err(e) => return Err(Error::Deserialization(e, details)),
        }
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
    #[error("{0}")]
    ApiEmptyResponse(ErrorDetails),
    #[error("{1}: {0}")]
    Deserialization(#[source] serde_json::Error, ErrorDetails),
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
        let resp = ApiServerResponse { code: "gateway-deleted".to_owned(), ..Default::default() };

        let error: ApiError = serde_json::from_str(&serde_json::to_string(&resp).unwrap()).unwrap();
        assert!(matches!(error, ApiError::GatewayDeleted));
    }

    #[test]
    fn invalid_credentials() {
        let resp =
            ApiServerResponse { code: "invalid-credentials".to_owned(), ..Default::default() };

        let error: ApiError = serde_json::from_str(&serde_json::to_string(&resp).unwrap()).unwrap();
        assert!(matches!(error, ApiError::InvalidCredentials));
    }
}
