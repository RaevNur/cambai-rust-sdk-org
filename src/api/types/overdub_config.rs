pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverdubConfig {
    /// Proportion of the original audio (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_audio_gain: Option<Option<f64>>,
    /// Proportion of the background audio (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_audio_gain: Option<Option<f64>>,
    /// Cross-fade duration in seconds (0.0 to 5.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_time: Option<f64>,
    /// Proportion of the fallback audio (0.0 to 1.0) (for streaming only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_audio_gain: Option<Option<f64>>,
}
