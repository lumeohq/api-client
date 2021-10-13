use std::net::IpAddr;

use chrono::{DateTime, Utc};
use fn_error_context::context;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct GatewayData {
    pub status: String,
    pub name: String,
    pub model: Option<String>,
    pub ip_local: Option<String>,
    pub ip_ext: Option<String>,
    pub mac_address: Option<String>,
}

#[derive(Serialize)]
pub struct NewGateway {
    pub application_id: Uuid,
    #[serde(flatten)]
    pub data: GatewayData,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Gateway {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub application_id: Uuid,
    pub status: String,
    pub name: String,
    pub model: Option<String>,
    pub ip_local: Option<String>,
    pub ip_ext: Option<String>,
    pub mac_address: Option<String>,
    pub access_token: String,
}

impl Client {
    #[context("Creating gateway (name={})", gateway.data.name)]
    pub async fn create_gateway(
        &self,
        application_id: Uuid,
        gateway: &NewGateway,
    ) -> anyhow::Result<Gateway> {
        self.post(&format!("/v1/apps/{}/devices", application_id), gateway).await
    }

    #[context("Updating local gateway IP")]
    pub async fn update_gateway_ip_local(&self, ip: &IpAddr) -> anyhow::Result<()> {
        self.put_text(
            &format!("/v1/apps/{}/devices/{}/ip_local", self.application_id()?, self.gateway_id()?),
            ip,
        )
        .await
    }

    #[context("Updating external gateway IP")]
    pub async fn update_gateway_ip_ext(&self, ip: &IpAddr) -> anyhow::Result<()> {
        self.put_text(
            &format!("/v1/apps/{}/devices/{}/ip_ext", self.application_id()?, self.gateway_id()?),
            ip,
        )
        .await
    }
}
