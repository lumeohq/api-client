use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackProperties {
    pub tracker: Tracker,
    #[serde(default)]
    pub profile: TrackerProfile,
    pub custom_properties: Option<TrackerCustomProperties>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tracker {
    /// Discriminative Correlation Filter (DCF) tracker uses a correlation filter-based
    /// online discriminative learning algorithm, coupled with a Hungarian algorithm for
    /// data association in multi-object tracking.
    Dcf,

    /// Kanade Lucas Tomasi (KLT) algorithm
    Klt,

    /// The Intersection of Union (IOU) tracker uses the intersection of the detector’s
    /// bounding boxes across frames to determine the object’s unique ID.
    Iou,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackerProfile {
    /// Loads the default tracker configuration
    Default,

    /// If available for this tracker type, loads a configuration tuned for stationary objects
    Stationary,

    /// Configuration tweaked by user
    Custom,
}

impl Default for TrackerProfile {
    fn default() -> Self {
        TrackerProfile::Default
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TrackerCustomProperties {
    /// NvDCF - used with 'Custom' TrackerProfile
    ///
    /// Min Intersection over Union (IoU) difference to existing tracks for discarding a new track.
    pub min_iou_diff: Option<f32>,
    /// Number of initial consecutive frames until the track is considered to be valid.
    pub activation_age: Option<NonZeroU32>,
}
