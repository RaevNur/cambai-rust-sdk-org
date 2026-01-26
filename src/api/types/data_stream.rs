pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DataStream {
    pub index: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_long_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag_string: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
}
