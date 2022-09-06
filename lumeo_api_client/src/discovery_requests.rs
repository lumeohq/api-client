use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use super::{cameras::CameraData, Client};
use crate::Result;

#[derive(Deserialize, Serialize)]
pub struct DiscoveryRequest {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub application_id: Uuid,
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
    pub async fn put_discovery_response(
        &self,
        request_id: Uuid,
        data: &DiscoveryRequestData,
    ) -> Result<()> {
        let application_id = self.application_id()?;
        let gateway_id = self.gateway_id()?;
        self.put_without_response_deserialization(
            &format!(
                "/v1/apps/{application_id}/gateways/{gateway_id}/discovery_request/{request_id}"
            ),
            Some(data),
        )
        .await
    }
}
