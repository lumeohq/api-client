use anyhow::Context;
use error::ResultExt;
use fn_error_context::context;
use reqwest::{header, Method, Url};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

pub mod apps;
pub mod cameras;
pub mod deployments;
pub mod discovery_requests;
mod error;
pub mod events;
pub mod files;
pub mod gateways;
pub mod orgs;
pub mod snapshots;
pub mod streams;

pub use error::HttpError;
pub type HttpResult<T> = Result<T, HttpError>;

pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    auth_token: String,
    application_id: Option<Uuid>,
    gateway_id: Option<Uuid>,
}

impl Client {
    pub fn new(
        base_url: String,
        auth_token: String,
        application_id: Option<Uuid>,
        gateway_id: Option<Uuid>,
    ) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url,
            auth_token,
            application_id,
            gateway_id,
        }
    }

    pub async fn get<T, Q>(&self, path: &str, query: Option<&Q>) -> HttpResult<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let query =
            query.map(serde_urlencoded::to_string).transpose().http_context(Method::GET, path)?;
        let request_builder = self.request(Method::GET, path, query.as_deref())?;

        async move { request_builder.send().await?.error_for_status()?.json().await }
            .await
            .http_context(Method::GET, path)
    }

    pub async fn post<T, R>(&self, path: &str, body: &R) -> HttpResult<T>
    where
        R: Serialize,
        T: DeserializeOwned,
    {
        let request_builder = self.request(Method::POST, path, None)?.json(body);
        async move { request_builder.send().await?.error_for_status()?.json().await }
            .await
            .http_context(Method::POST, path)
    }

    pub async fn put<T, R>(&self, path: &str, body: &R) -> HttpResult<T>
    where
        R: Serialize,
        T: DeserializeOwned,
    {
        let request_builder = self.request(Method::PUT, path, None)?.json(body);
        async move { request_builder.send().await?.error_for_status()?.json().await }
            .await
            .http_context(Method::PUT, path)
    }

    pub async fn put_without_response_deserialization<R>(
        &self,
        path: &str,
        body: &R,
    ) -> HttpResult<()>
    where
        R: Serialize,
    {
        let request_builder = self.request(Method::PUT, path, None)?;
        async move { request_builder.json(body).send().await?.error_for_status() }
            .await
            .http_context(Method::PUT, path)?;

        Ok(())
    }

    pub async fn put_text<R>(&self, path: &str, body: &R) -> HttpResult<()>
    where
        R: ToString + ?Sized,
    {
        let request_builder = self.request(Method::PUT, path, None)?;
        async move { request_builder.body(body.to_string()).send().await?.error_for_status() }
            .await
            .http_context(Method::PUT, path)?;

        Ok(())
    }

    pub async fn delete(&self, path: &str) -> HttpResult<()> {
        let request_builder = self.request(Method::DELETE, path, None)?;
        async move { request_builder.send().await?.error_for_status() }
            .await
            .http_context(Method::DELETE, path)?;

        Ok(())
    }

    pub fn request(
        &self,
        method: Method,
        path: &str,
        query: Option<&str>,
    ) -> HttpResult<reqwest::RequestBuilder> {
        let mut url =
            Url::parse(&format!("{}{}", self.base_url, path)).http_context(method.clone(), path)?;

        if let Some(q2) = query {
            let full_query = match url.query() {
                Some(q1) => format!("{}&{}", q1, q2),
                None => q2.to_owned(),
            };

            url.set_query(Some(&full_query));
        }

        Ok(self
            .http_client
            .request(method, url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.auth_token)))
    }

    #[context("Getting application ID")]
    fn application_id(&self) -> anyhow::Result<Uuid> {
        self.application_id.context("application_id not specified")
    }

    #[context("Getting gateway ID")]
    fn gateway_id(&self) -> anyhow::Result<Uuid> {
        self.gateway_id.context("gateway_id not specified")
    }
}
