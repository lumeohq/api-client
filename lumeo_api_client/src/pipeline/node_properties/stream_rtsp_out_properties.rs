use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamRtspOutProperties {
    #[serde(flatten)]
    pub runtime: Option<StreamRtspOutRuntime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamRtspOutRuntime {
    /// RTSP stream URI.
    ///
    /// This field is set to `Some` by `lumeod`.
    // TODO: this should be set by API server, more details here:
    //       https://app.clubhouse.io/lumeo/story/940/set-correct-stream-url-for-pipeline-streams-created-for-webrtc-nodes
    pub uri: Url,

    /// UDP port for `udpsink` element.
    ///
    /// This field is set to `Some` by `lumeod`
    ///
    /// Since pipeline can have multiple RTSP output streams we need to
    /// distribute ports at `lumeod` level.
    pub udp_port: Option<u16>,

    /// Stream ID.
    ///
    /// This field is set by API server.
    ///
    /// Stream ID is used by `lumeod` to add a WebRTC endpoint to webrtcstreamer service.
    pub stream_id: Uuid,
}

impl_stream_props!(StreamRtspOutProperties, StreamRtspOutRuntime, "rtsp");
