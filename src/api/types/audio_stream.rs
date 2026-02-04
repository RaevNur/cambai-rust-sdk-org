pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioStream {
    pub index: i64,
    pub codec_name: String,
    pub codec_long_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag_string: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_tag: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Option<String>>,
    pub sample_fmt: String,
    pub sample_rate: i64,
    pub channels: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_layout: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<HashMap<String, serde_json::Value>>>,
}
