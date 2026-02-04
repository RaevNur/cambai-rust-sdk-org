pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DubbingResult {
    pub audio_url: String,
    pub transcript: Vec<Transcript>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<Option<String>>,
}
