use std::{collections::BTreeMap, num::NonZeroU32};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::pipeline::Resolution;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelInferenceProperties {
    /// The ID of the inference model.
    pub model_id: Uuid,

    /// Infer only on objects another previously detected by this node.
    pub infer_on_node: Option<String>,

    /// Inference interval (frames).
    #[serde(default = "default_inference_interval")]
    pub inference_interval: NonZeroU32,

    #[serde(flatten)]
    pub runtime: Option<ModelInferenceRuntime>,

    /// Maps each class label (key) to the ClassInferenceProperties set.
    /// Use "*" as key to specify global properties that should affect all the model classes.
    #[serde(default)]
    pub class_properties: Option<BTreeMap<String, ClassInferenceProperties>>,
}

fn default_inference_interval() -> NonZeroU32 {
    NonZeroU32::new(1).unwrap()
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ModelInferenceRuntime {
    /// Path to the inferencing node config.
    ///
    /// This field is set to `Some` by `lumeod`.
    ///
    /// Inferencing GStreamer element requires config to be stored locally
    /// so `lumeod` should download everything required for model to run
    /// and generate a config.
    pub config: Option<std::path::PathBuf>,

    /// Inferencing Gstreamer element requires a unique numerical id for itself
    /// and for any other inferencing nodes it operates on.
    /// Map key is pipeline node id, val is unique id assigned to it at runtime.
    pub infer_node_unique_ids: Option<BTreeMap<String, i32>>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClassInferenceProperties {
    /// Minimum inference threshold, should be set on the [0.0, 1.0] interval.
    pub min_inference_threshold: Option<f32>,
    /// Relative difference between sides of the rectangles to merge them into a group.
    ///
    /// Used in OpenCV groupRectangles function and DBSCAN algorithm.
    pub eps: Option<f32>,
    /// Minimum size (WidthxHeight format) in pixels to consider a detected object.
    pub object_min_size: Option<Resolution>,
    /// Maximum size (WidthxHeight format) in pixels to consider a detected object.
    pub object_max_size: Option<Resolution>,
}
