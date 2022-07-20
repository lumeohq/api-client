use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub source: SnapshotSource,
    pub file_id: Uuid,
    pub put_url: Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SnapshotSource {
    Camera { camera_id: Uuid },
    Stream { stream_id: Uuid },
}
