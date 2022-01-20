use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Client;
use crate::Result;

#[derive(Default, Serialize)]
pub struct SnapshotParams {
    pub gateway_id: Option<Uuid>,
}

#[derive(Default, Deserialize)]
pub struct SnapshotResponse {
    pub file_id: Uuid,
}

impl Client {
    pub async fn take_camera_snapshot(&self, camera_id: Uuid) -> Result<SnapshotResponse> {
        let application_id = self.application_id()?;
        self.post(
            &format!("/v1/apps/{application_id}/cameras/{camera_id}/snapshot"),
            &SnapshotParams::default(),
        )
        .await
    }

    pub async fn take_stream_snapshot(&self, stream_id: Uuid) -> Result<SnapshotResponse> {
        let application_id = self.application_id()?;
        self.post(
            &format!("/v1/apps/{application_id}/streams/{stream_id}/snapshot"),
            &SnapshotParams::default(),
        )
        .await
    }

    pub async fn set_camera_snapshot_file_id(
        &self,
        camera_id: Uuid,
        snapshot_file_id: Uuid,
    ) -> Result<()> {
        let application_id = self.application_id()?;
        self.put_text(
            &format!("/v1/apps/{application_id}/cameras/{camera_id}/snapshot_file_id"),
            &snapshot_file_id.to_hyphenated(),
        )
        .await
    }

    pub async fn set_stream_snapshot_file_id(
        &self,
        stream_id: Uuid,
        snapshot_file_id: Uuid,
    ) -> Result<()> {
        let application_id = self.application_id()?;
        self.put_text(
            &format!("/v1/apps/{application_id}/streams/{stream_id}/snapshot_file_id"),
            &snapshot_file_id.to_hyphenated(),
        )
        .await
    }
}
