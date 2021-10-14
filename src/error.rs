use std::fmt;

use reqwest::Method;

#[derive(Debug)]
pub struct HttpError {
    inner: InnerHttpError,
    method: Method,
    path: String,
}

impl HttpError {
    pub fn request_method(&self) -> &Method {
        &self.method
    }

    pub fn request_path(&self) -> &str {
        &self.path
    }

    /// If the error comes from `reqwest`, return the inner `reqwest::Error`.
    pub fn to_reqwest_error(&self) -> Option<&reqwest::Error> {
        match &self.inner {
            InnerHttpError::Url(_) | InnerHttpError::Query(_) => None,
            InnerHttpError::Reqwest(e) => Some(e),
        }
    }

    // Url and Query accessors left out because there's no use case yet
}

#[derive(Debug)]
enum InnerHttpError {
    Url(url::ParseError),
    Query(serde_urlencoded::ser::Error),
    Reqwest(reqwest::Error),
}

pub(crate) trait ResultExt<T> {
    fn http_context(self, method: Method, path: &str) -> Result<T, HttpError>;
}

impl fmt::Display for InnerHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InnerHttpError::Url(e) => e.fmt(f),
            InnerHttpError::Query(e) => e.fmt(f),
            InnerHttpError::Reqwest(e) => e.fmt(f),
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} request to `{}` failed: {}", self.method, self.path, self.inner)
    }
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match &self.inner {
            InnerHttpError::Url(e) => e,
            InnerHttpError::Query(e) => e,
            InnerHttpError::Reqwest(e) => e,
        })
    }
}

impl<T> ResultExt<T> for Result<T, url::ParseError> {
    fn http_context(self, method: Method, path: &str) -> Result<T, HttpError> {
        self.map_err(|e| HttpError { inner: InnerHttpError::Url(e), method, path: path.into() })
    }
}

impl<T> ResultExt<T> for Result<T, serde_urlencoded::ser::Error> {
    fn http_context(self, method: Method, path: &str) -> Result<T, HttpError> {
        self.map_err(|e| HttpError { inner: InnerHttpError::Query(e), method, path: path.into() })
    }
}

impl<T> ResultExt<T> for Result<T, reqwest::Error> {
    fn http_context(self, method: Method, path: &str) -> Result<T, HttpError> {
        self.map_err(|e| HttpError { inner: InnerHttpError::Reqwest(e), method, path: path.into() })
    }
}
