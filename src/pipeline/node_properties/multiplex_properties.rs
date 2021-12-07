//! Interleave two or more streams and create a single "multiplexed" stream, to optimize processing.
//! Also resizes all frames to specified output resolution.

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::pipeline::Resolution;

#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiplexProperties {
    pub resolution: Option<Resolution>,
    pub num_streams: u32,
}
