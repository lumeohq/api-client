use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use strum::AsRefStr;
use url::Url;
use uuid::Uuid;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputStreamRuntime {
    Rtsp(InputRtspStreamRuntime),
    WebRtc(InputWebRtcStreamRuntime),
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
