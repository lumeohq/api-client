use std::fmt;

use serde::{Deserialize, Serialize};

mod result_serde;
use result_serde::{BoolResponseCompat, StructResponseCompat};

/// WebRTC client -> server requests
#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    /// Get offer from streamer
    GetOffer(GetOfferRequest),
    /// Set offer answer from client
    SetAnswer(SetAnswerRequest),
    /// Add client's ICE candidate
    AddCandidate(AddCandidateRequest),
    /// Get remote ICE candidates from streamer
    GetCandidates(GetCandidatesRequest),
    /// Force hang up (close client's streaming session)
    HangUp(HangUpRequest),
}

/// Request offer form the streamer
#[derive(Debug, Serialize, Deserialize)]
pub struct GetOfferRequest {
    /// Stream UUID
    pub stream_id: String,
}

/// ICE server entry
///
/// Refer to https://developer.mozilla.org/en-US/docs/Web/API/RTCIceServer
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IceServer {
    /// ICE servers URL's array. Note that payload is not an URI, it's format is like "stun:stun.l.google.com:19302"
    pub urls: Vec<String>,
    /// Credential (TURN only)
    pub credential: Option<String>,
    /// Username (TURN only)
    pub username: Option<String>,
}

/// Response on GetOfferRequest
#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "StructResponseCompat<Offer>")]
pub struct GetOfferResponse(pub Result<Offer, WebRtcError>);

/// Success result of GetOfferResponse
#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    /// Peer id identifier, should be user in all future calls
    pub peer_id: String,
    /// ICE servers settings for peer connection construction
    #[serde(rename = "iceServers")]
    pub ice_servers: Vec<IceServer>,
    /// SDP payload
    pub sdp: String,
    /// SDP type
    pub r#type: String,
}

/// Set offer answer request from client
#[derive(Serialize, Deserialize, Debug)]
pub struct SetAnswerRequest {
    /// SDP payload
    pub sdp: String,
    /// SDP type
    pub r#type: String,
    /// peer id (received from URL)
    pub peer_id: String,
}

/// Boolean response on SDP answer request
#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "BoolResponseCompat")]
pub struct SetAnswerResponse(pub Result<(), WebRtcError>);

/// ICE candidate.
///
/// Refer to https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IceCandidate {
    pub candidate: String,
    pub sdp_m_line_index: u32,
    pub sdp_mid: String,
}

/// Set ICE candidate request
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCandidateRequest {
    /// ICE candidate
    pub candidate: IceCandidate,
    /// peer id (received from URL)
    pub peer_id: String,
}

/// Boolean response on set candidate request
#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "BoolResponseCompat")]
pub struct AddCandidateResponse(pub Result<(), WebRtcError>);

/// Get remote candidates list request
#[derive(Serialize, Deserialize, Debug)]
pub struct GetCandidatesRequest {
    /// peer id (received from URL)
    pub peer_id: String,
}

/// Response on get ICE candidates request
#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "StructResponseCompat<Candidates>")]
pub struct GetCandidatesResponse(pub Result<Candidates, WebRtcError>);

/// Success result of GetCandidatesResponse
#[derive(Serialize, Deserialize, Debug)]
pub struct Candidates {
    /// Is the list complete flag
    /// Request MUST make another GetCandidatesRequest request till "completed" flag isn't true
    /// (poll for remote candidates)
    pub completed: bool,
    /// Remote ICE candidates list, every entry must be appended to the client's peer connection
    pub candidates: Vec<IceCandidate>,
}

/// Hang up request
#[derive(Serialize, Deserialize, Debug)]
pub struct HangUpRequest {
    /// peer id (received from URL)
    pub peer_id: String,
}

/// Hang up response
#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "BoolResponseCompat")]
pub struct HangUpResponse(pub Result<(), WebRtcError>);

/// Responses from streamer
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    GetOffer(GetOfferResponse),
    SetAnswer(SetAnswerResponse),
    AddCandidate(AddCandidateResponse),
    GetCandidates(GetCandidatesResponse),
    HangUp(HangUpResponse),
}

/// Error in handling of a WebRTC command
#[derive(Serialize, Deserialize, Debug)]
pub struct WebRtcError(Option<String>);

impl fmt::Display for WebRtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(err) => f.write_str(err),
            // FIXME: Make the inner error string mandatory once we don't use
            //        WebRTC-streamer anymore.
            None => f.write_str("<no error message from WebRTC-streamer>"),
        }
    }
}

impl WebRtcError {
    /// Create a WebRtcError with the given error message.
    pub fn new(msg: String) -> Self {
        Self(Some(msg))
    }

    /// Create a WebRtcError from the given error.
    ///
    /// Shorthand for `WebRtcError::new(err.to_string())`.
    pub fn from_error(err: impl std::error::Error) -> Self {
        Self(Some(err.to_string()))
    }

    /// Create a WebRtcError without a specific error message.
    ///
    /// Meant to be used when WebRTC-streamer returns null, false, or an object
    /// without an error property where an object is not the expected response.
    pub fn null() -> Self {
        Self(None)
    }
}

impl std::error::Error for WebRtcError {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{GetOfferResponse, IceServer, Offer, Response};

    #[test]
    fn response_serialization() {
        let res = Response::GetOffer(GetOfferResponse(Ok(Offer {
            peer_id: "1927468303".into(),
            ice_servers: vec![
                IceServer {
                    urls: vec!["stun:stun.l.google.com:19302".into()],
                    credential: None,
                    username: None,
                },
                IceServer {
                    urls: vec!["turn:traverse.lumeo.com".into()],
                    credential: Some("video".into()),
                    username: Some("lumeo".into()),
                },
            ],
            sdp: "...".into(),
            r#type: "offer".into(),
        })));

        assert_eq!(
            serde_json::to_value(res).unwrap(),
            json!({
                "GetOffer": {
                    "Ok": {
                        "peer_id": "1927468303",
                        "iceServers": [
                            {
                                "urls": ["stun:stun.l.google.com:19302"],
                                "credential": null,
                                "username": null
                            },
                            {
                                "urls": ["turn:traverse.lumeo.com"],
                                "credential": "video",
                                "username": "lumeo"
                            }
                        ],
                        "sdp": "...",
                        "type": "offer",
                    }
                }
            })
        );
    }
}
