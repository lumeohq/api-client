use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "location", rename_all = "snake_case")]
pub enum ClipProperties {
    Local(LocalClipProperties),
    LumeoCloud(LumeoCloudClipProperties),
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonClipProperties {
    pub min_duration: Option<u64>,
    pub max_duration: Option<u64>,
    pub max_size: Option<u64>,
    pub retention_duration: Option<u64>,
    pub webhook_url: Option<Url>,
    pub trigger: Option<String>,
    #[serde(default)]
    pub trigger_mode: TriggerMode,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerMode {
    Exact,
    FixedDuration,
}

impl Default for TriggerMode {
    fn default() -> Self {
        TriggerMode::Exact
    }
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalClipProperties {
    #[serde(flatten)]
    pub common: CommonClipProperties,
    pub path: Option<PathBuf>,
    pub max_edge_files: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LumeoCloudClipProperties {
    #[serde(flatten)]
    pub common: CommonClipProperties,
}
