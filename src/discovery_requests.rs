use chrono::{DateTime, Utc};
use fn_error_context::context;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use super::{cameras::CameraData, Client};

#[derive(Deserialize, Serialize)]
pub struct DiscoveryRequest {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub application_id: Uuid,
    #[serde(rename = "device_id", alias = "gateway_id")]
    pub gateway_id: Uuid,
    #[serde(flatten)]
    pub result: DiscoveryResult,
}

#[derive(Deserialize, Serialize)]
pub struct DiscoveryRequestData {
    #[serde(flatten)]
    pub result: DiscoveryResult,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "status", content = "result")]
pub enum DiscoveryResult {
    Pending,
    Success(Vec<CameraData>),
    Error(JsonValue),
}

impl Client {
    #[context("Responding to discovery request {}", request_id)]
    pub async fn put_discovery_response(
        &self,
        request_id: Uuid,
        data: &DiscoveryRequestData,
    ) -> anyhow::Result<()> {
        Ok(self
            .put_without_response_deserialization(
                &format!(
                    "/v1/apps/{}/devices/{}/discovery_request/{}",
                    self.application_id()?,
                    self.gateway_id()?,
                    request_id
                ),
                data,
            )
            .await?)
    }
}
