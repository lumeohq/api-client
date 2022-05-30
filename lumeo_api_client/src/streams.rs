use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
use url::Url;
use uuid::Uuid;
use vec1::Vec1;

use super::Client;
use crate::Result;

#[derive(Serialize)]
pub struct StreamData {
    pub name: Option<String>,
    pub source: StreamSource,
    pub stream_type: StreamType,
    #[serde(rename = "device_id")]
    pub gateway_id: Option<Uuid>,
    #[serde(flatten)]
    pub content: StreamContent,
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
    #[serde(alias = "device_id")]
    pub gateway_id: Option<Uuid>,
    #[serde(flatten)]
    pub content: StreamContent,
    pub status: StreamStatus,
    pub camera_id: Option<Uuid>,
    pub deployment_id: Option<Uuid>,
    pub node: Option<String>,
    pub configuration: Option<String>,
    pub snapshot_file_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamContent {
    Urls { urls: Vec1<Url> },
    Files { file_ids: Vec1<Uuid> },
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

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;
    use vec1::vec1;

    use super::*;

    #[test]
    fn stream_data_should_serialize_with_file_ids() {
        let file_id = Uuid::new_v4();
        let data = StreamData {
            name: None,
            source: StreamSource::UriStream,
            stream_type: StreamType::File,
            gateway_id: None,
            content: StreamContent::Files { file_ids: vec1![file_id] },
            status: None,
            camera_id: None,
            deployment_id: None,
            node: None,
            configuration: None,
            snapshot_file_id: None,
        };

        let expected_json = json!({
            "name": null,
            "source": "uri_stream",
            "stream_type": "file",
            "device_id": null,
            "file_ids": [file_id],
            "status": null,
            "camera_id": null,
            "deployment_id": null,
            "node": null,
            "configuration": null,
            "snapshot_file_id": null,
        });

        assert_json_eq!(expected_json, data);
    }

    #[test]
    fn stream_data_should_deserialize_with_file_ids() {
        let file_id = Uuid::new_v4();
        let json = json!({
            "name": null,
            "source": "uri_stream",
            "stream_type": "file",
            "device_id": null,
            "status": "online",
            "file_ids": [file_id],
            "status": null,
            "camera_id": null,
            "deployment_id": null,
            "node": null,
            "configuration": null,
            "snapshot_file_id": null,
        });

        let expected_data = StreamData {
            name: None,
            source: StreamSource::UriStream,
            stream_type: StreamType::File,
            gateway_id: None,
            content: StreamContent::Files { file_ids: vec1![file_id] },
            status: None,
            camera_id: None,
            deployment_id: None,
            node: None,
            configuration: None,
            snapshot_file_id: None,
        };

        assert_json_eq!(expected_data, json);
    }

    #[test]
    fn stream_data_should_serialize_with_urls() {
        let url = Url::parse("file:///example.mp4").expect("Failed to parse URL");
        let data = StreamData {
            name: None,
            source: StreamSource::UriStream,
            stream_type: StreamType::File,
            gateway_id: None,
            content: StreamContent::Urls { urls: vec1![url.clone()] },
            status: None,
            camera_id: None,
            deployment_id: None,
            node: None,
            configuration: None,
            snapshot_file_id: None,
        };

        let expected_json = json!({
            "name": null,
            "source": "uri_stream",
            "stream_type": "file",
            "device_id": null,
            "urls": [url],
            "status": null,
            "camera_id": null,
            "deployment_id": null,
            "node": null,
            "configuration": null,
            "snapshot_file_id": null,
        });

        assert_json_eq!(expected_json, data);
    }

    #[test]
    fn stream_data_should_deserialize_with_urls() {
        let url = Url::parse("file:///example.mp4").expect("Failed to parse URL");
        let json = json!({
            "name": null,
            "source": "uri_stream",
            "stream_type": "file",
            "device_id": null,
            "status": "online",
            "urls": [url],
            "status": null,
            "camera_id": null,
            "deployment_id": null,
            "node": null,
            "configuration": null,
            "snapshot_file_id": null,
        });

        let expected_data = StreamData {
            name: None,
            source: StreamSource::UriStream,
            stream_type: StreamType::File,
            gateway_id: None,
            content: StreamContent::Urls { urls: vec1![url] },
            status: None,
            camera_id: None,
            deployment_id: None,
            node: None,
            configuration: None,
            snapshot_file_id: None,
        };

        assert_json_eq!(expected_data, json);
    }
}
