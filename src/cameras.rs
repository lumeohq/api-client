use chrono::{DateTime, Utc};
use fn_error_context::context;
use lumeo_commands::api::camera::{Camera as DiscoveredCamera, Status};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;
use url::Url;
use uuid::Uuid;

use super::{streams::Stream, Client};

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
    #[context("Reading camera {}", camera_id)]
    pub async fn read_camera(&self, camera_id: Uuid) -> anyhow::Result<Camera> {
        Ok(self
            .get(&format!("/v1/apps/{}/cameras/{}", self.application_id()?, camera_id), None::<&()>)
            .await?)
    }

    #[context("Listing cameras")]
    pub async fn list_cameras(&self) -> anyhow::Result<Vec<Camera>> {
        Ok(self.get(&format!("/v1/apps/{}/cameras", self.application_id()?), None::<&()>).await?)
    }

    #[context("Listing camera streams")]
    pub async fn list_camera_streams(&self, camera_id: Uuid) -> anyhow::Result<Vec<Stream>> {
        Ok(self
            .get(
                &format!("/v1/apps/{}/cameras/{}/streams", self.application_id()?, camera_id),
                None::<&()>,
            )
            .await?)
    }

    #[context("Updating camera {}", camera_id)]
    pub async fn update_camera(
        &self,
        camera_id: Uuid,
        data: &CameraData,
    ) -> anyhow::Result<Camera> {
        let path = format!("/v1/apps/{}/cameras/{}", self.application_id()?, camera_id);
        Ok(self.put(&path, data).await?)
    }

    #[context("Setting cameras statuses")]
    pub async fn set_cameras_statuses(&self, cameras: &[CameraData]) -> anyhow::Result<()> {
        Ok(self
            .put_without_response_deserialization(
                &format!(
                    "/v1/apps/{}/devices/{}/cameras_statuses",
                    self.application_id()?,
                    self.gateway_id()?
                ),
                &cameras,
            )
            .await?)
    }

    #[context("Setting camera status")]
    pub async fn set_camera_status(&self, camera_id: Uuid, status: &str) -> anyhow::Result<()> {
        Ok(self
            .put_text(
                &format!("/v1/apps/{}/cameras/{}/status", self.application_id()?, camera_id),
                status,
            )
            .await?)
    }
}

impl From<&DiscoveredCamera> for CameraData {
    fn from(c: &DiscoveredCamera) -> Self {
        match c {
            DiscoveredCamera::Local(camera) => CameraData {
                status: match camera.status {
                    Status::Online => Some("online".to_string()),
                    Status::Offline => Some("offline".to_string()),
                },
                name: camera.name.clone(),
                model: camera.model.clone(),
                conn_type: Some("local".to_string()),
                gateway_id: None,
                uri: Some(camera.uri.clone()),
                ip_local: None,
                ip_ext: None,
                mac_address: None,
                username: None,
                password: None,
                configuration: None,
                capabilities: serde_json::to_value(&camera.capabilities).ok(),
                snapshot_file_id: None,
            },
            DiscoveredCamera::Remote(camera) => CameraData {
                status: match camera.status {
                    Status::Online => Some("online".to_string()),
                    Status::Offline => Some("offline".to_string()),
                },
                name: camera.name.clone(),
                model: camera.model.clone(),
                conn_type: Some("remote".to_string()),
                gateway_id: None,
                uri: Some(camera.uri.clone()),
                ip_local: camera.ip_local.clone(),
                ip_ext: None,
                mac_address: Some(camera.mac_address.clone()),
                username: None,
                password: None,
                configuration: None,
                capabilities: None,
                snapshot_file_id: None,
            },
        }
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
