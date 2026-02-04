pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SourceStream {
    pub language: Languages,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<StreamCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streamid: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_streams: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_audio_stream: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_input: Option<Option<bool>>,
}
