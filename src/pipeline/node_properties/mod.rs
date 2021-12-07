//! This module contains all kinds of Lumeo pipeline nodes

use serde::{Deserialize, Serialize};

pub mod clip_properties;
pub mod demultiplex_properties;
pub mod encode_properties;
pub mod function_properties;
pub mod grid_properties;
pub mod gst_template_properties;
pub mod metadata_inserter_properties;
pub mod model_inference_properties;
pub mod multiplex_properties;
pub mod overlay_properties;
pub mod snapshot_properties;
#[macro_use]
pub mod stream_properties;
pub mod stream_rtsp_out_properties;
pub mod stream_web_rtc_out_properties;
pub mod track_properties;
pub mod transform_properties;
pub mod video_source_properties;

pub use clip_properties::{
    ClipProperties, CommonClipProperties, LocalClipProperties, LumeoCloudClipProperties,
};
pub use demultiplex_properties::DemultiplexProperties;
pub use encode_properties::EncodeProperties;
pub use function_properties::{FunctionProperties, FunctionRuntime};
pub use grid_properties::GridProperties;
pub use gst_template_properties::GstTemplateProperties;
pub use metadata_inserter_properties::MetadataInserterProperties;
pub use model_inference_properties::{
    ClassInferenceProperties, ModelInferenceProperties, ModelInferenceRuntime,
};
pub use multiplex_properties::MultiplexProperties;
pub use overlay_properties::OverlayProperties;
pub use snapshot_properties::{
    CommonSnapshotProperties, LocalSnapshotProperties, LumeoCloudSnapshotProperties,
    SnapshotProperties,
};
pub use stream_properties::{StreamProperties, StreamRuntime};
pub use stream_rtsp_out_properties::{StreamRtspOutProperties, StreamRtspOutRuntime};
pub use stream_web_rtc_out_properties::{StreamWebRtcOutProperties, StreamWebRtcOutRuntime};
pub use track_properties::{TrackProperties, Tracker, TrackerProfile};
pub use transform_properties::{FlipDirection, TransformProperties};
pub use video_source_properties::{
    CameraProperties, CameraRuntime, CommonVideoSourceProperties, InputRtspStreamRuntime,
    InputStreamProperties, InputStreamRuntime, InputWebRtcStreamRuntime, RotateDirection,
    UsbCameraRuntime, VideoSourceProperties,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeProperties {
    Clip(ClipProperties),
    Demultiplex(DemultiplexProperties),
    Encode(EncodeProperties),
    Function(FunctionProperties),
    Grid(GridProperties),
    GstTemplate(GstTemplateProperties),
    #[serde(rename = "metadata_add")]
    MetadataInserter(MetadataInserterProperties),
    ModelInference(ModelInferenceProperties),
    Multiplex(MultiplexProperties),
    Overlay(OverlayProperties),
    Snapshot(SnapshotProperties),
    StreamRtspOut(StreamRtspOutProperties),
    #[serde(rename = "stream_webrtc_out")]
    StreamWebRtcOut(StreamWebRtcOutProperties),
    Track(TrackProperties),
    Transform(TransformProperties),
    #[serde(rename = "video")]
    VideoSource(VideoSourceProperties),
}
