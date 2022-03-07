use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Client, Result};

#[derive(Deserialize, Serialize)]
pub struct VideoSourceMetric {
    /// Time when the collection period has started
    pub start: DateTime<Utc>,
    pub deployment_id: Uuid,
    /// Camera or stream id
    pub source_id: Uuid,
    /// `camera` or `stream`
    pub source_type: String,
    /// ID of the pipeline's VideoSource node
    pub node_id: String,
    /// Duration of this collection period
    pub duration_in_ms: i32,
    /// Duration of video streamed in this collection period (for streams processed faster than realtime)
    pub streamed_ms: i32,
    /// Number of bytes of the uncompressed video streamed in this collection period
    pub streamed_bytes: i64,
}

impl Client {
    pub async fn push_video_source_metric(
        &self,
        gateway_id: Uuid,
        metric: &VideoSourceMetric,
    ) -> Result<()> {
        self.post_without_response_deserialization(
            &format!("/metrics/v1/gateways/{gateway_id}/video_source_metrics"),
            metric,
        )
        .await
    }
}
