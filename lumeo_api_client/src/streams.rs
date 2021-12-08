use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::Client;
use crate::Result;

#[derive(Serialize)]
pub struct StreamData {
    pub name: Option<String>,
    pub source: String,
    pub stream_type: String,
    #[serde(rename = "device_id")]
    pub gateway_id: Option<Uuid>,
    pub uri: Option<Url>,
    pub status: Option<String>,
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
    pub source: String,
    pub stream_type: String,
    #[serde(alias = "device_id")]
    pub gateway_id: Option<Uuid>,
    pub uri: Option<Url>,
    pub status: String,
    pub camera_id: Option<Uuid>,
    pub deployment_id: Option<Uuid>,
    pub node: Option<String>,
    pub configuration: Option<String>,
    pub snapshot_file_id: Option<Uuid>,
}

impl Client {
    pub async fn create_stream(&self, stream: &StreamData) -> Result<Stream> {
        self.post(&format!("/v1/apps/{}/streams", self.application_id()?), stream).await
    }

    pub async fn read_stream(&self, stream_id: Uuid) -> Result<Stream> {
        self.get(&format!("/v1/apps/{}/streams/{}", self.application_id()?, stream_id), None::<&()>)
            .await
    }
}
