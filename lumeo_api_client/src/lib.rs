use std::time::Duration;

use error::ResultExt;
use reqwest::{header, Method, Url};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

pub mod apps;
pub mod auth;
pub mod cameras;
pub mod commands;
pub mod deployments;
pub mod discovery_requests;
pub mod error;
pub mod events;
pub mod files;
pub mod gateways;
pub mod metrics;
pub mod models;
pub mod orgs;
pub mod pipeline;
pub mod snapshots;
pub mod streams;

use error::{verify_response, Error};
type Callback = Box<dyn Fn(&Error) + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_LOGIN_TIMEOUT: Duration = Duration::from_secs(30);

pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    auth_token: String,
    application_id: Option<Uuid>,
    gateway_id: Option<Uuid>,
    error_cb: Option<Callback>,
}

impl Client {
    pub fn new(
        base_url: String,
        auth_token: String,
        application_id: Option<Uuid>,
        gateway_id: Option<Uuid>,
        user_agent: Option<&str>,
    ) -> reqwest::Result<Self> {
        Ok(Self::with_http_client(
            base_url,
            auth_token,
            application_id,
            gateway_id,
            reqwest::Client::builder()
                .timeout(DEFAULT_TIMEOUT)
                .user_agent(user_agent.unwrap_or("Lumeo api-client"))
                .build()?,
        ))
    }

    pub fn with_http_client(
        base_url: String,
        auth_token: String,
        application_id: Option<Uuid>,
        gateway_id: Option<Uuid>,
        http_client: reqwest::Client,
    ) -> Self {
        Self { http_client, base_url, auth_token, application_id, gateway_id, error_cb: None }
    }

    pub async fn get<T, Q>(&self, path: &str, query: Option<&Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.get_internal(path, query).await.map_err(|err| self.through_cb(err))
    }

    async fn get_internal<T, Q>(&self, path: &str, query: Option<&Q>) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let query =
            query.map(serde_urlencoded::to_string).transpose().http_context(Method::GET, path)?;
        let request_builder = self.request(Method::GET, path, query.as_deref())?;

        verify_response(request_builder.send().await, Method::GET, path)
            .await?
            .json()
            .await
            .http_context(Method::GET, path)
    }

    pub async fn post<T, R>(&self, path: &str, body: &R) -> Result<T>
    where
        R: Serialize,
        T: DeserializeOwned,
    {
        self.post_internal(path, body).await.map_err(|err| self.through_cb(err))
    }

    pub async fn post_without_response_deserialization<R>(
        &self,
        path: &str,
        body: Option<&R>,
    ) -> Result<()>
    where
        R: Serialize,
    {
        self.request_without_response_deserialization_internal(Method::POST, path, body)
            .await
            .map_err(|err| self.through_cb(err))
    }

    async fn post_internal<T, R>(&self, path: &str, body: &R) -> Result<T>
    where
        R: Serialize,
        T: DeserializeOwned,
    {
        let request_builder = self.request(Method::POST, path, None)?.json(body);

        verify_response(request_builder.send().await, Method::POST, path)
            .await?
            .json()
            .await
            .http_context(Method::POST, path)
    }

    pub async fn put<T, R>(&self, path: &str, body: &R) -> Result<T>
    where
        R: Serialize,
        T: DeserializeOwned,
    {
        self.put_internal(path, body).await.map_err(|err| self.through_cb(err))
    }

    async fn put_internal<T, R>(&self, path: &str, body: &R) -> Result<T>
    where
        R: Serialize,
        T: DeserializeOwned,
    {
        let request_builder = self.request(Method::PUT, path, None)?.json(body);
        verify_response(request_builder.send().await, Method::PUT, path)
            .await?
            .json()
            .await
            .http_context(Method::PUT, path)
    }

    pub async fn put_without_response_deserialization<R>(
        &self,
        path: &str,
        body: Option<&R>,
    ) -> Result<()>
    where
        R: Serialize,
    {
        self.request_without_response_deserialization_internal(Method::PUT, path, body)
            .await
            .map_err(|err| self.through_cb(err))
    }

    async fn request_without_response_deserialization_internal<R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&R>,
    ) -> Result<()>
    where
        R: Serialize,
    {
        let mut request_builder =
            self.request(method.clone(), path, None).map_err(|err| self.through_cb(err))?;

        if let Some(body) = body {
            request_builder = request_builder.json(body);
        }

        if method == Method::POST && body.is_none() {
            // See https://github.com/seanmonstar/reqwest/issues/838
            request_builder = request_builder.header(header::CONTENT_LENGTH, 0)
        }

        verify_response(request_builder.send().await, method, path)
            .await
            .map_err(|err| self.through_cb(err))?;
        Ok(())
    }

    pub async fn put_text<R>(&self, path: &str, body: &R) -> Result<()>
    where
        R: ToString + ?Sized,
    {
        self.put_text_internal(path, body).await.map_err(|err| self.through_cb(err))
    }

    async fn put_text_internal<R>(&self, path: &str, body: &R) -> Result<()>
    where
        R: ToString + ?Sized,
    {
        let request_builder = self.request(Method::PUT, path, None)?;
        verify_response(request_builder.body(body.to_string()).send().await, Method::PUT, path)
            .await?;

        Ok(())
    }

    pub async fn delete<Q>(&self, path: &str, query: Option<&Q>) -> Result<()>
    where
        Q: Serialize,
    {
        self.delete_internal(path, query).await.map_err(|err| self.through_cb(err))
    }

    async fn delete_internal<Q>(&self, path: &str, query: Option<&Q>) -> Result<()>
    where
        Q: Serialize,
    {
        let query = query
            .map(serde_urlencoded::to_string)
            .transpose()
            .http_context(Method::DELETE, path)?;
        let request_builder = self.request(Method::DELETE, path, query.as_deref())?;
        verify_response(request_builder.send().await, Method::DELETE, path).await?;

        Ok(())
    }

    fn request(
        &self,
        method: Method,
        path: &str,
        query: Option<&str>,
    ) -> Result<reqwest::RequestBuilder> {
        self.request_internal(method, path, query).map_err(|err| self.through_cb(err))
    }

    fn request_internal(
        &self,
        method: Method,
        path: &str,
        query: Option<&str>,
    ) -> Result<reqwest::RequestBuilder> {
        let mut url =
            Url::parse(&format!("{}{}", self.base_url, path)).http_context(method.clone(), path)?;

        if let Some(q2) = query {
            let full_query = match url.query() {
                Some(q1) => format!("{q1}&{q2}"),
                None => q2.to_owned(),
            };

            url.set_query(Some(&full_query));
        }

        Ok(self
            .http_client
            .request(method, url)
            .header(header::AUTHORIZATION, format!("Bearer {}", self.auth_token)))
    }

    pub fn register_error_cb(&mut self, cb: impl Fn(&Error) + Send + Sync + 'static) {
        self.error_cb = Some(Box::new(cb));
    }

    fn through_cb(&self, err: Error) -> Error {
        if let Some(cb) = &self.error_cb {
            cb(&err);
        }
        err
    }

    fn application_id(&self) -> Result<Uuid> {
        self.application_id.ok_or(Error::ApplicationIdMissing).map_err(|err| self.through_cb(err))
    }

    fn gateway_id(&self) -> Result<Uuid> {
        self.gateway_id.ok_or(Error::GatewayIdMissing).map_err(|err| self.through_cb(err))
    }
}
