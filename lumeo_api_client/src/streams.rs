use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
use url::Url;
use uuid::Uuid;

use super::Client;
use crate::Result;

#[derive(Serialize)]
pub struct StreamData {
    pub name: Option<String>,
    pub source: StreamSource,
    pub stream_type: StreamType,
    pub gateway_id: Option<Uuid>,
    pub uri: Url,
    pub status: Option<StreamStatus>,
    pub camera_id: Option<Uuid>,
    pub deployment_id: Option<Uuid>,
    pub node: Option<String>,
    pub configuration: Option<String>,
    pub snapshot_file_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct Stream {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub application_id: Uuid,
    pub name: String,
    pub source: StreamSource,
    pub stream_type: StreamType,
    pub gateway_id: Option<Uuid>,
    pub uri: Option<Url>,
    pub status: StreamStatus,
    pub camera_id: Option<Uuid>,
    pub deployment_id: Option<Uuid>,
    pub node: Option<String>,
    pub configuration: Option<String>,
    pub snapshot_file_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Debug, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "stream_type", rename_all = "snake_case"))]
pub enum StreamType {
    Rtsp,
    Webrtc,
    File,
}

#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "stream_source", rename_all = "snake_case"))]
pub enum StreamSource {
    CameraStream,
    UriStream,
    PipelineStream,
}

#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Debug, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "stream_status", rename_all = "snake_case"))]
pub enum StreamStatus {
    Online,
    Offline,
    Unknown,
}

impl Client {
    pub async fn create_stream(&self, stream: &StreamData) -> Result<Stream> {
        let application_id = self.application_id()?;
        self.post(&format!("/v1/apps/{application_id}/streams"), stream).await
    }

    pub async fn read_stream(&self, stream_id: Uuid) -> Result<Stream> {
        let application_id = self.application_id()?;
        self.get(&format!("/v1/apps/{application_id}/streams/{stream_id}"), None::<&()>).await
    }
}
