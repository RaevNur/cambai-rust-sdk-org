pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TargetStream {
    pub languages: Vec<Languages>,
    pub url: String,
    pub r#type: StreamType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streamid: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<Option<Vec<Option<i64>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_video: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_subtitles: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channel_layout: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_bitrate: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_output: Option<Option<bool>>,
}
