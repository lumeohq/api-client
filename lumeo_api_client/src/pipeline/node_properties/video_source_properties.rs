use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use strum::AsRefStr;
use url::Url;
use uuid::Uuid;
use vec1::Vec1;

use crate::pipeline::{
    resolution::Resolution,
    transform_properties::{Crop, FlipDirection},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AsRefStr)]
#[serde(tag = "source_type", rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum VideoSourceProperties {
    Camera(CameraProperties),
    Stream(InputStreamProperties),
}

impl VideoSourceProperties {
    pub fn uri(&self) -> Option<&Url> {
        use VideoSourceProperties::*;
        match self {
            Camera(CameraProperties { runtime, .. }) => Some(runtime.as_ref()?.uri()),
            Stream(InputStreamProperties { runtime, .. }) => runtime.as_ref()?.uri(),
        }
    }

    pub fn name(&self) -> Option<&str> {
        use VideoSourceProperties::*;
        match self {
            Camera(CameraProperties { runtime, .. }) => Some(runtime.as_ref()?.name()),
            Stream(InputStreamProperties { runtime, .. }) => runtime.as_ref()?.name(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CommonVideoSourceProperties {
    /// ID of associated object.
    /// - if source_type=='camera' then source_id is camera ID
    /// - if source_type=='stream' then source_id is stream ID
    pub source_id: Uuid,

    /// Resolution of video source.
    /// If unset then some reasonable default is used.
    pub resolution: Option<Resolution>,

    /// Framerate of video source.
    /// If unset then some reasonable default is used.
    pub framerate: Option<u32>,

    /// Rotate video source by arbitrary angle.
    /// Does not change resolution but parts of the initial image can be left outside of the frame.
    pub rotate: Option<f64>,

    /// Rotate video source by 90/180/270 degrees.
    /// Can change resolution but does not lose pixels.
    pub rotate_fixed_angle: Option<RotateDirection>,

    /// Flip video source.
    pub flip: Option<FlipDirection>,

    /// Crop video source.
    /// Format: "left:right:top:bottom".
    pub crop: Option<Crop>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RotateDirection {
    /// Rotate clockwise by 90 degrees.
    /// Changes resolution, for example 640x480 becomes 480x640.
    Clockwise90,

    /// Rotate by 180 degrees.
    #[serde(alias = "counter_clockwise180")]
    Clockwise180,

    /// Rotate counter-clockwise by 90 degrees.
    /// Changes resolution, for example 640x480 becomes 480x640.
    CounterClockwise90,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraProperties {
    #[serde(flatten)]
    pub common: CommonVideoSourceProperties,
    #[serde(flatten)]
    pub runtime: Option<CameraRuntime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputStreamProperties {
    #[serde(flatten)]
    pub common: CommonVideoSourceProperties,
    #[serde(flatten)]
    pub runtime: Option<InputStreamRuntime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraRuntime {
    Usb(UsbCameraRuntime),
    Csi(CsiCameraRuntime),
}

impl CameraRuntime {
    pub fn uri(&self) -> &Url {
        use CameraRuntime::*;
        match self {
            Usb(UsbCameraRuntime { uri, .. }) | Csi(CsiCameraRuntime { uri, .. }) => uri,
        }
    }

    pub fn name(&self) -> &str {
        use CameraRuntime::*;
        match self {
            Usb(UsbCameraRuntime { name, .. }) | Csi(CsiCameraRuntime { name, .. }) => name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputStreamRuntime {
    /// Non-Realtime stream from URLs (e.g. `http`, `https`, and `file`)
    UrlFile(InputUrlFileStreamRuntime),
    /// Non-Realtime stream from files in the Lumeo API (`lumeo://files/<file_id>`. e.g. saved clips)
    LumeoFile(InputLumeoFileStreamRuntime),
    /// Despite the name, this currently represents any URI stream that should
    /// be interpreted as realtime. Including `rtsp`, `http`, `https` and `file`.
    /// Note that even if the stream isn't actually realtime,
    /// it will be treated by billing as such.
    Rtsp(InputRtspStreamRuntime),
    WebRtc(InputWebRtcStreamRuntime),
}

impl InputStreamRuntime {
    pub fn uri(&self) -> Option<&Url> {
        use InputStreamRuntime::*;
        match self {
            LumeoFile(_) => None,
            Rtsp(InputRtspStreamRuntime { uri, .. }) => Some(uri),
            UrlFile(InputUrlFileStreamRuntime { urls, .. }) => match urls.as_slice() {
                [url] => Some(url),
                // For more than one it isn't clear which one to use, so choose none
                _ => None,
            },
            WebRtc(_) => None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        use InputStreamRuntime::*;
        match self {
            LumeoFile(InputLumeoFileStreamRuntime { name, .. }) => Some(name),
            Rtsp(InputRtspStreamRuntime { name, .. }) => Some(name),
            UrlFile(InputUrlFileStreamRuntime { name, .. }) => Some(name),
            WebRtc(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsbCameraRuntime {
    /// Local USB camera URI.
    ///
    /// Example: "file:///dev/video0"
    pub uri: Url,

    /// Camera name.
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsiCameraRuntime {
    /// Local CSI camera URI.
    ///
    /// Example: "file:///dev/video0"
    pub uri: Url,

    /// Camera name.
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputRtspStreamRuntime {
    /// RTSP stream URI.
    ///
    /// Example: "rtsp://192.168.0.42:554/hd_stream"
    pub uri: Url,

    /// Stream name.
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputWebRtcStreamRuntime {
    // TODO: define how do we use WebRTC streams as inputs
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputUrlFileStreamRuntime {
    /// Stream name.
    pub name: String,
    /// URLs like e.g. `http`, `https` or `file`.
    /// Always has at least one element.
    pub urls: Vec1<Url>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputLumeoFileStreamRuntime {
    /// Stream name.
    pub name: String,
    /// Lumeo file ids.
    /// Alaways has at least one element.
    pub file_ids: Vec1<Uuid>,
}

// FIXME: replace manual deserialization with
//  ```
//      #[serde(alias = "fps")]`
//      pub framerate: Option<u32>,
//  ```
//  when serde bug is fixed:
//  https://github.com/serde-rs/serde/issues/1504
impl<'de> Deserialize<'de> for CommonVideoSourceProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            source_id: Uuid,
            resolution: Option<Resolution>,
            framerate: Option<u32>,
            fps: Option<u32>,
            rotate: Option<f64>,
            rotate_fixed_angle: Option<RotateDirection>,
            flip: Option<FlipDirection>,
            crop: Option<Crop>,
        }

        let Helper {
            source_id,
            resolution,
            framerate,
            fps,
            rotate,
            rotate_fixed_angle,
            flip,
            crop,
        } = Deserialize::deserialize(deserializer)?;

        Ok(Self {
            source_id,
            resolution,
            framerate: framerate.or(fps),
            rotate,
            rotate_fixed_angle,
            flip,
            crop,
        })
    }
}
