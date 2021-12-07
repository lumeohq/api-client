use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncodeProperties {
    pub codec: String,
    pub max_bitrate: Option<u32>,
    pub bitrate: Option<u32>,
    pub quality: Option<u32>,
    #[serde(alias = "fps")]
    pub framerate: Option<u32>,
}
