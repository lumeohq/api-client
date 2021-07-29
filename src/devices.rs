use std::net::IpAddr;

use chrono::{DateTime, Utc};
use fn_error_context::context;
use log_derive::logfn;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::Client;

#[skip_serializing_none]
#[derive(Serialize)]
pub struct DeviceData {
    pub status: String,
    pub name: String,
    pub model: Option<String>,
    pub ip_local: Option<String>,
    pub ip_ext: Option<String>,
    pub mac_address: Option<String>,
}

#[derive(Serialize)]
pub struct NewDevice {
    pub application_id: Uuid,
    #[serde(flatten)]
    pub data: DeviceData,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Device {
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
    #[context("Creating device (name={})", device.data.name)]
    pub async fn create_device(
        &self,
        application_id: Uuid,
        device: &NewDevice,
    ) -> anyhow::Result<Device> {
        self.post(&format!("/v1/apps/{}/devices", application_id), device).await
    }

    #[logfn(err = "WARN")]
    pub async fn update_device_ip_local(&self, ip: &IpAddr) -> anyhow::Result<()> {
        self.put_text(
            &format!("/v1/apps/{}/devices/{}/ip_local", self.application_id()?, self.device_id()?),
            ip,
        )
        .await
    }

    #[logfn(err = "WARN")]
    pub async fn update_device_ip_ext(&self, ip: &IpAddr) -> anyhow::Result<()> {
        self.put_text(
            &format!("/v1/apps/{}/devices/{}/ip_ext", self.application_id()?, self.device_id()?),
            ip,
        )
        .await
    }
}
