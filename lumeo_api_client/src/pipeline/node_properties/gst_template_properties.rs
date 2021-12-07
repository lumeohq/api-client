use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GstTemplateProperties {
    pub definition: String,
    #[serde(default)]
    pub props: Map<String, Value>,
}
