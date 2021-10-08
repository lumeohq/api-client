use chrono::{DateTime, Utc};
use fn_error_context::context;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::Client;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileCloudStatus {
    Disabled,
    Uploading,
    Uploaded,
}

#[derive(Debug, Serialize)]
pub struct FileData {
    pub name: String,
    pub size: i64,
    pub duration: Option<i32>,
    pub cloud_status: FileCloudStatus,
    #[serde(rename = "device_id")]
    pub gateway_id: Option<Uuid>,
    pub local_path: Option<String>,
    pub pipeline_id: Option<Uuid>,
    pub node_id: Option<String>,
    pub deployment_id: Option<Uuid>,
    pub camera_id: Option<Uuid>,
    pub stream_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub size: i64,
    pub duration: Option<i32>,
    pub cloud_status: FileCloudStatus,
    #[serde(rename = "device_id")]
    pub gateway_id: Option<Uuid>,
    pub local_path: Option<String>,
    pub application_id: Uuid,
    pub pipeline_id: Option<Uuid>,
    pub node_id: Option<String>,
    pub deployment_id: Option<Uuid>,
    pub camera_id: Option<Uuid>,
    pub stream_id: Option<Uuid>,
    pub data_url: Option<Url>,
    pub metadata_url: Option<Url>,
}

#[derive(Debug, Default, Serialize)]
pub struct ListParams {
    /// Maximum number of files to return
    pub limit: i16,
    /// Filter: Lower bound for creation time (inclusive)
    pub created_ts_since: Option<DateTime<Utc>>,
    /// Filter: Upper bound for creation time (exclusive)
    pub created_ts_until: Option<DateTime<Utc>>,
    /// Filter: Node ID(s)
    pub node_ids: Vec<String>,
    /// Filter: Deployment ID(s)
    pub deployment_ids: Vec<Uuid>,
    /// Filter: Camera ID(s)
    pub camera_ids: Vec<Uuid>,
    /// Filter: Stream ID(s)
    pub stream_ids: Vec<Uuid>,
    /// Filter: Gateway ID(s)
    #[serde(rename = "device_id")]
    pub gateway_ids: Vec<Uuid>,
    /// Filter: Pipeline ID(s)
    pub pipeline_ids: Vec<Uuid>,
}

impl Client {
    #[context("Listing files")]
    pub async fn list_files(&self, params: Option<&ListParams>) -> anyhow::Result<Vec<File>> {
        self.get(&format!("/v1/apps/{}/files", self.application_id()?), params).await
    }

    #[context("Creating file {}", file_data.name)]
    pub async fn create_file(&self, file_data: &FileData) -> anyhow::Result<File> {
        self.post(&format!("/v1/apps/{}/files", self.application_id()?), file_data).await
    }

    #[context("Reading file {}", id)]
    pub async fn read_file(&self, id: Uuid) -> anyhow::Result<File> {
        self.get(&format!("/v1/apps/{}/files/{}", self.application_id()?, id), None::<&()>).await
    }

    #[context("Updating file {}", id)]
    pub async fn update_file(&self, id: Uuid, file_data: &FileData) -> anyhow::Result<File> {
        self.put(&format!("/v1/apps/{}/files/{}", self.application_id()?, id), file_data).await
    }

    #[context("Deleting file {}", id)]
    pub async fn delete_file(&self, id: Uuid) -> anyhow::Result<()> {
        self.delete(&format!("/v1/apps/{}/files/{}", self.application_id()?, id)).await
    }
}
