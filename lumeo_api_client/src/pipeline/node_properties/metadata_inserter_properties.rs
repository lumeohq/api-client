use serde::{Deserialize, Serialize};
use url::Url;

/// Insert metadata frames from an external source.
///
/// Reads lines (newline delimited) of TEXT from the given URI and appends them to the frames. The
/// only supported scheme currently is `lumeo`, which will use TCP as transport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetadataInserterProperties {
    #[serde(rename = "source_uri")]
    pub uri: Url,
}
