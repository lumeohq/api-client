use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;
use crate::{cameras::Camera, Result};

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
    pub async fn create_gateway(
        &self,
        application_id: Uuid,
        gateway: &NewGateway,
    ) -> Result<Gateway> {
        self.post(&format!("/v1/apps/{application_id}/gateways"), gateway).await
    }

    pub async fn read_gateway(&self) -> Result<Gateway> {
        let application_id = self.application_id()?;
        let gateway_id = self.gateway_id()?;
        self.get(&format!("/v1/apps/{application_id}/gateways/{gateway_id}"), None::<&()>).await
    }

    pub async fn list_linked_cameras(&self) -> Result<Vec<Camera>> {
        let application_id = self.application_id()?;
        let gateway_id = self.gateway_id()?;
        self.get(
            &format!("/v1/apps/{application_id}/gateways/{gateway_id}/linked_cameras"),
            None::<&()>,
        )
        .await
    }

    pub async fn update_gateway_ip_local(&self, ip: &IpAddr) -> Result<()> {
        let application_id = self.application_id()?;
        let gateway_id = self.gateway_id()?;
        self.put_text(&format!("/v1/apps/{application_id}/gateways/{gateway_id}/ip_local"), ip)
            .await
    }

    pub async fn update_gateway_ip_ext(&self, ip: &IpAddr) -> Result<()> {
        let application_id = self.application_id()?;
        let gateway_id = self.gateway_id()?;
        self.put_text(&format!("/v1/apps/{application_id}/gateways/{gateway_id}/ip_ext"), ip).await
    }
}
