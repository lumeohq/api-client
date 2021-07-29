use anyhow::Context;
use fn_error_context::context;
use reqwest::{header, Method, Url};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

pub mod apps;
pub mod cameras;
pub mod deployments;
pub mod devices;
pub mod discovery_requests;
pub mod events;
pub mod files;
pub mod orgs;
pub mod snapshots;
pub mod streams;

mod util;

pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    auth_token: String,
    application_id: Option<Uuid>,
    device_id: Option<Uuid>,
}

impl Client {
    pub fn new(
        base_url: String,
        auth_token: String,
        application_id: Option<Uuid>,
        device_id: Option<Uuid>,
    ) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url,
            auth_token,
            application_id,
            device_id,
        }
    }

    #[context("Get {}", path)]
    pub async fn get<T, Q>(&self, path: &str, query: Option<&Q>) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let query = query.map(serde_urlencoded::to_string).transpose()?;
        Ok(self
            .request(Method::GET, path, query.as_deref())?
            .send()
            .await
            .with_context(|| format!("Error GETting API object from {:?}", path))?
            .error_for_status()
            .with_context(|| format!("Error GETting API object from {:?}", path))?
            .json()
            .await?)
    }

    #[context("Post {}", path)]
    pub async fn post<T, R>(&self, path: &str, body: &R) -> anyhow::Result<T>
    where
        R: Serialize,
        T: DeserializeOwned,
    {
        Ok(self
            .request(Method::POST, path, None)?
            .json(body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    #[context("Put without deserializing response {}", path)]
    pub async fn put_without_response_deserialization<R>(
        &self,
        path: &str,
        body: &R,
    ) -> anyhow::Result<()>
    where
        R: Serialize,
    {
        self.request(Method::PUT, path, None)?.json(body).send().await?.error_for_status()?;

        Ok(())
    }

    #[context("Put text {}", path)]
    pub async fn put_text<R>(&self, path: &str, body: &R) -> anyhow::Result<()>
    where
        R: ToString + ?Sized,
    {
        self.request(Method::PUT, path, None)?
            .body(body.to_string())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    #[context("Delete {}", path)]
    pub async fn delete(&self, path: &str) -> anyhow::Result<()> {
        self.request(Method::DELETE, path, None)?.send().await?.error_for_status()?;

        Ok(())
    }

    pub fn request(
        &self,
        method: Method,
        path: &str,
        query: Option<&str>,
    ) -> anyhow::Result<reqwest::RequestBuilder> {
        let mut url = Url::parse(&format!("{}{}", self.base_url, path))?;

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

    #[context("Getting device ID")]
    fn device_id(&self) -> anyhow::Result<Uuid> {
        self.device_id.context("device_id not specified")
    }
}
