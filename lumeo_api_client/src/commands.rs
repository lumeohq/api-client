use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use thiserror::Error;

pub mod camera;
pub mod deployment;
pub mod snapshot;
pub mod webrtc;

/// Error types
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize command")]
    DeserializeCommand,
}

/// API message type
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// Request
    Request(Request),
    /// Notification
    Notification(Notification),
}

impl Message {
    pub fn trace_headers(&self) -> Option<&TraceHeaders> {
        match self {
            Message::Request(Request { trace_headers, .. })
            | Message::Notification(Notification { trace_headers, .. }) => trace_headers.as_ref(),
        }
    }
}

/// Request type
///
/// Expect response from remote end
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// Request body
    pub body: Body,
    /// Response correlation string
    pub respond_to: String,
    /// Trace headers used for distributed tracing.
    pub trace_headers: Option<TraceHeaders>,
}

/// Notification
///
/// Fire and forget packet
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    /// Notification body
    pub body: Body,
    /// Trace headers used for distributed tracing.
    pub trace_headers: Option<TraceHeaders>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TraceHeaders(pub Vec<(String, String)>);

impl TraceHeaders {
    pub fn as_iter_str(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

/// API message body payloads
#[derive(Debug, Serialize, Deserialize)]
pub enum Body {
    // TODO: Remove the comment about restart once all deployed lumeod instances support `RestartDeployment`
    /// Start pipeline deployment (also triggers restarts on lumeod instances that don't support `RestartDeployment` yet)
    StartDeployment(deployment::StartDeployment),
    /// Restart pipeline deployment
    RestartDeployment(deployment::RestartDeployment),
    /// Stop deployment
    StopDeployment(deployment::StopDeployment),
    /// Camera commands
    Camera(camera::Request),
    /// WebRTC subcommands collection
    WebRtc(webrtc::Request),
    /// Snapshot commands
    Snapshot(snapshot::Request),
    /// Delete gateway command
    DeleteGateway,
}
