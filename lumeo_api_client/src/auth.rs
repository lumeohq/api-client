use serde::{Deserialize, Serialize};
use url::Url;

use super::Client;
use crate::{
    error::ErrorDetails, verify_response, Error::Reqwest, Method, Result, ResultExt,
    DEFAULT_LOGIN_TIMEOUT,
};

#[derive(Serialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

impl Client {
    // This is an associated function rather than a method because `Client` is unfortunately build
    // in a way that it needs to have an auth token to be created. We can't create a `Client`
    // without having done a login first. For that to work we need to call the associated function
    // `Client::login()` first in order to obtain a token and create `Client` from the returned token.
    pub async fn login(server_address: String, login_params: LoginParams) -> Result<LoginResponse> {
        let path = "/v1/internal/auth/login";
        let url =
            Url::parse(&format!("{server_address}{path}")).http_context(Method::POST, path)?;

        let raw_reqwest_client =
            reqwest::Client::builder().timeout(DEFAULT_LOGIN_TIMEOUT).build().map_err(|e| {
                let status = e.status();
                Reqwest(e, ErrorDetails { method: Method::POST, path: path.to_owned(), status })
            })?;

        let request_builder = raw_reqwest_client.post(url).json(&login_params);
        verify_response(request_builder.send().await, Method::POST, path)
            .await?
            .json()
            .await
            .http_context(Method::POST, path)
    }
}
