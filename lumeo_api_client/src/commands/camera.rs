use num_rational::Rational32;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Discover cameras
    Discover { request_id: Uuid },

    /// Create camera streams
    CreateStreams { camera_id: Uuid },

    /// Fetch camera details from physical camera and update the camera
    UpdateDetails { camera_id: Uuid },
}

/// `Camera` type represents different camera types.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "conn_type", rename_all = "snake_case")]
pub enum Camera {
    Local(LocalCamera),
    Remote(RemoteCamera),
}

/// `LocalCamera` type represents cameras connected directly to the edge gateway via USB or CSI.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct LocalCamera {
    /// URI to access camera, for example `file:///dev/video0`
    pub uri: Url,

    /// Status
    pub status: Status,

    /// Name
    pub name: Option<String>,

    /// Manufacturer
    pub manufacturer: Option<String>,

    /// Model
    pub model: Option<String>,

    /// Camera's physical interface
    pub interface: LocalCameraInterface,

    /// List of camera capabilities
    pub capabilities: Vec<Capability>,
}

/// `RemoteCamera` type represents IP cameras.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct RemoteCamera {
    /// ONVIF device management service URI used to access this camera,
    /// for example `http://192.168.0.42/device`
    pub uri: Url,

    /// MAC address
    pub mac_address: String,

    /// Status
    pub status: Status,

    /// Name
    pub name: Option<String>,

    /// Manufacturer
    pub manufacturer: Option<String>,

    /// Model
    pub model: Option<String>,

    /// Local IP address
    pub ip_local: Option<String>,

    /// List of camera streams
    pub streams: Vec<Stream>,
}

/// IP camera stream.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Stream {
    /// URL to access this stream, for example `rtsp://192.168.0.42:554/hd_stream`
    pub rtsp_uri: Url,

    /// Stream name
    pub name: String,

    /// Stream capability
    pub capability: Capability,
}

/// Configuration parameters that are supported by this specific camera.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Capability {
    /// Capability name
    pub name: String,

    /// Width in pixels
    pub width: i32,

    /// Height in pixels
    pub height: i32,

    /// List of supported framerates
    pub framerates: Vec<Rational32>,

    /// Video format, "MJPG", "YUYV", etc.
    pub format: Option<String>,
}

/// Camera status.
#[derive(Serialize, Deserialize, AsRefStr, Display, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Status {
    Online,
    Offline,
    Unauthorized,
}

/// Local camera interface.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LocalCameraInterface {
    Usb,
    Csi,
}

macro_rules! for_each_variant {
    ($self:ident, $var:ident, $action:expr) => {
        match $self {
            Camera::Local($var) => $action,
            Camera::Remote($var) => $action,
        }
    };
}

impl Camera {
    pub fn uri(&self) -> &Url {
        for_each_variant!(self, camera, &camera.uri)
    }

    pub fn set_uri(&mut self, uri: Url) {
        for_each_variant!(self, camera, camera.uri = uri)
    }

    pub fn status(&self) -> &Status {
        for_each_variant!(self, camera, &camera.status)
    }

    pub fn set_status(&mut self, status: Status) {
        for_each_variant!(self, camera, camera.status = status)
    }

    pub fn name(&self) -> Option<&str> {
        for_each_variant!(self, camera, camera.name.as_deref())
    }

    pub fn set_name(&mut self, name: Option<String>) {
        for_each_variant!(self, camera, camera.name = name)
    }

    pub fn manufacturer(&self) -> Option<&str> {
        for_each_variant!(self, camera, camera.manufacturer.as_deref())
    }

    pub fn set_manufacturer(&mut self, manufacturer: Option<String>) {
        for_each_variant!(self, camera, camera.manufacturer = manufacturer)
    }

    pub fn model(&self) -> Option<&str> {
        for_each_variant!(self, camera, camera.model.as_deref())
    }

    pub fn set_model(&mut self, model: Option<String>) {
        for_each_variant!(self, camera, camera.model = model)
    }
}

#[test]
fn basic_de() {
    let json = r#"{
        "conn_type": "local",
        "uri": "file:///dev/video0",
        "status": "online",
        "name": "Entrance #4",
        "interface": "usb",
        "capabilities": []
    }"#;

    let actual: Camera = serde_json::from_str(json).unwrap();

    let expected = Camera::Local(LocalCamera {
        uri: "file:///dev/video0".parse().unwrap(),
        status: Status::Online,
        name: Some("Entrance #4".into()),
        manufacturer: None,
        model: None,
        interface: LocalCameraInterface::Usb,
        capabilities: vec![],
    });

    assert_eq!(actual, expected);
}

#[test]
fn status_to_string() {
    assert_eq!(Status::Offline.as_ref(), "offline");
    assert_eq!(Status::Online.as_ref(), "online");
    assert_eq!(Status::Unauthorized.as_ref(), "unauthorized");
}
