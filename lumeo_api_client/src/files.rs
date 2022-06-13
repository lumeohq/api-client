use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use super::Client;
use crate::Result;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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

impl File {
    pub fn stream_url(&self) -> Url {
        create_lumeo_file_url(self.id)
    }
}

pub fn create_lumeo_file_url(file_id: Uuid) -> Url {
    format!("lumeo://{file_id}").parse().unwrap_or_else(|error| {
        unreachable!("Failed to parse generated Lumeo URL with error: {error}")
    })
}

pub fn parse_file_id_from_lumeo_url(url: &Url) -> std::result::Result<Uuid, FileUrlError> {
    if url.scheme() != "lumeo" {
        return Err(FileUrlError::InvalidScheme(url.scheme().into()));
    }

    let file_id = url.host_str().ok_or(FileUrlError::MissingHost)?;

    if url.as_str().len() != ("lumeo://".len() + file_id.len()) {
        return Err(FileUrlError::InvalidLength);
    }

    Uuid::parse_str(file_id).map_err(|_| FileUrlError::InvalidUuid(file_id.into()))
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FileUrlError {
    #[error("Invalid scheme, expected 'lumeo', got '{0}'")]
    InvalidScheme(String),
    #[error("Missing host")]
    MissingHost,
    #[error("Invalid length")]
    InvalidLength,
    #[error("Invalid UUID: '{0}'")]
    InvalidUuid(String),
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
    pub gateway_ids: Vec<Uuid>,
    /// Filter: Pipeline ID(s)
    pub pipeline_ids: Vec<Uuid>,
}

pub type DeleteParams = ListParams;

impl Client {
    pub async fn list_files(&self, params: Option<&ListParams>) -> Result<Vec<File>> {
        let application_id = self.application_id()?;
        self.get(&format!("/v1/apps/{application_id}/files"), params).await
    }

    pub async fn create_file(&self, file_data: &FileData) -> Result<File> {
        let application_id = self.application_id()?;
        self.post(&format!("/v1/apps/{application_id}/files"), file_data).await
    }

    pub async fn read_file(&self, file_id: Uuid) -> Result<File> {
        let application_id = self.application_id()?;
        self.get(&format!("/v1/apps/{application_id}/files/{file_id}"), None::<&()>).await
    }

    pub async fn update_file(&self, file_id: Uuid, file_data: &FileData) -> Result<File> {
        let application_id = self.application_id()?;
        self.put(&format!("/v1/apps/{application_id}/files/{file_id}"), file_data).await
    }

    pub async fn update_cloud_status(
        &self,
        file_id: Uuid,
        cloud_status: &FileCloudStatus,
    ) -> Result<()> {
        let application_id = self.application_id()?;
        self.put_text(
            &format!("/v1/apps/{application_id}/files/{file_id}/cloud_status"),
            cloud_status,
        )
        .await
    }

    pub async fn delete_file(&self, file_id: Uuid) -> Result<()> {
        let application_id = self.application_id()?;
        self.delete(&format!("/v1/apps/{application_id}/files/{file_id}"), None::<&()>).await
    }

    pub async fn delete_files(&self, params: &DeleteParams) -> Result<()> {
        let application_id = self.application_id()?;
        self.delete(&format!("/v1/apps/{application_id}/files"), Some(params)).await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_lumeo_file_url_from_id() {
        let file_id =
            Uuid::parse_str("f65c8128-e25a-11ec-b486-efa3b8212d7f").expect("Failed to parse UUID");
        let url = create_lumeo_file_url(file_id);

        assert_eq!("lumeo://f65c8128-e25a-11ec-b486-efa3b8212d7f", url.as_str());
    }

    #[test]
    fn should_parse_lumeo_file_url() {
        let url = Url::parse("lumeo://f65c8128-e25a-11ec-b486-efa3b8212d7f")
            .expect("Failed to parse URL");

        let file_id = parse_file_id_from_lumeo_url(&url).expect("Failed to parse lumeo file URL");
        let expected_file_id =
            Uuid::parse_str("f65c8128-e25a-11ec-b486-efa3b8212d7f").expect("Failed to parse UUID");

        assert_eq!(expected_file_id, file_id);
    }

    #[test]
    fn should_not_parse_lumeo_file_url_with_query() {
        let url = Url::parse("lumeo://f65c8128-e25a-11ec-b486-efa3b8212d7f?key=value")
            .expect("Failed to parse URL");

        assert_eq!(Err(FileUrlError::InvalidLength), parse_file_id_from_lumeo_url(&url));
    }

    #[test]
    fn should_not_parse_lumeo_file_url_without_host() {
        let url = Url::parse("lumeo://").expect("Failed to parse URL");

        assert_eq!(Err(FileUrlError::MissingHost), parse_file_id_from_lumeo_url(&url));
    }

    #[test]
    fn should_not_parse_lumeo_file_url_with_incorrect_scheme() {
        let url = Url::parse("http://example.com").expect("Failed to parse URL");

        assert_eq!(
            Err(FileUrlError::InvalidScheme("http".into())),
            parse_file_id_from_lumeo_url(&url)
        );
    }

    #[test]
    fn should_not_parse_lumeo_file_url_with_invalid_uuid() {
        let invalid_uuid = {
            let valid_uuid = "f65c8128-e25a-11ec-b486-efa3b8212d7f";
            Uuid::parse_str(valid_uuid).expect("Failed to parse valid UUID");
            valid_uuid.replace('f', "g")
        };
        Uuid::parse_str(&invalid_uuid).expect_err("UUID was not invalid");

        let url = Url::parse("lumeo://g65c8128-e25a-11ec-b486-efa3b8212d7f")
            .expect("Failed to parse URL");

        assert_eq!(
            Err(FileUrlError::InvalidUuid("g65c8128-e25a-11ec-b486-efa3b8212d7f".into())),
            parse_file_id_from_lumeo_url(&url)
        );
    }
}
