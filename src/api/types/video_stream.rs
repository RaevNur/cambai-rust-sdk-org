pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VideoStream {
    pub index: i64,
    pub codec_name: String,
    pub codec_long_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag_string: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avc: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
}
