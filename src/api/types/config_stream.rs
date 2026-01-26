pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigStream {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<ConfigStreamPipeline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixing: Option<OverdubConfig>,
}
