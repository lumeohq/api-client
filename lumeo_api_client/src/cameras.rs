use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;
use url::Url;
use uuid::Uuid;

use super::{streams::Stream, Client};
use crate::Result;

#[skip_serializing_none]
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CameraData {
    pub status: Option<String>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub conn_type: Option<String>,
    pub gateway_id: Option<Uuid>,
    pub uri: Option<Url>,
    pub ip_local: Option<String>,
    pub ip_ext: Option<String>,
    pub mac_address: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub configuration: Option<String>,
    pub capabilities: Option<JsonValue>,
    pub snapshot_file_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct NewCamera {
    pub application_id: Uuid,
    pub data: CameraData,
}

#[skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Camera {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub application_id: Uuid,
    pub status: String,
    pub name: String,
    pub model: Option<String>,
    pub conn_type: Option<String>,
    pub gateway_id: Option<Uuid>,
    pub uri: Option<Url>,
    pub ip_local: Option<String>,
    pub ip_ext: Option<String>,
    pub mac_address: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub configuration: Option<String>,
    pub capabilities: Option<JsonValue>,
    pub snapshot_file_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct DevicesCamera {
    pub id: Uuid,
    #[serde(alias = "device_id")]
    pub gateway_id: Uuid,
    pub camera_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct NewLinkedCamera {
    pub camera_id: Uuid,
}

impl Client {
    pub async fn read_camera(&self, camera_id: Uuid) -> Result<Camera> {
        self.get(&format!("/v1/apps/{}/cameras/{}", self.application_id()?, camera_id), None::<&()>)
            .await
    }

    pub async fn list_cameras(&self) -> Result<Vec<Camera>> {
        self.get(&format!("/v1/apps/{}/cameras", self.application_id()?), None::<&()>).await
    }

    pub async fn list_camera_streams(&self, camera_id: Uuid) -> Result<Vec<Stream>> {
        self.get(
            &format!("/v1/apps/{}/cameras/{}/streams", self.application_id()?, camera_id),
            None::<&()>,
        )
        .await
    }

    pub async fn update_camera(&self, camera_id: Uuid, data: &CameraData) -> Result<Camera> {
        let path = format!("/v1/apps/{}/cameras/{}", self.application_id()?, camera_id);
        self.put(&path, data).await
    }

    pub async fn set_cameras_statuses(&self, cameras: &[CameraData]) -> Result<()> {
        self.put_without_response_deserialization(
            &format!(
                "/v1/apps/{}/gateways/{}/cameras_statuses",
                self.application_id()?,
                self.gateway_id()?
            ),
            &cameras,
        )
        .await
    }

    pub async fn set_camera_status(&self, camera_id: Uuid, status: &str) -> Result<()> {
        self.put_text(
            &format!("/v1/apps/{}/cameras/{}/status", self.application_id()?, camera_id),
            status,
        )
        .await
    }
}

impl Camera {
    pub fn to_data(&self) -> CameraData {
        CameraData {
            status: Some(self.status.clone()),
            name: Some(self.name.clone()),
            model: self.model.clone(),
            conn_type: self.conn_type.clone(),
            gateway_id: self.gateway_id,
            uri: self.uri.clone(),
            ip_local: self.ip_local.clone(),
            ip_ext: self.ip_ext.clone(),
            mac_address: self.mac_address.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            configuration: self.configuration.clone(),
            capabilities: self.capabilities.clone(),
            snapshot_file_id: self.snapshot_file_id,
        }
    }
}
