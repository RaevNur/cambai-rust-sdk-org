pub use crate::prelude::*;

/// Configuration for the output audio of Streaming TTS.
///
/// Attributes:
/// format: The format of the output audio. Defaults to MP3.
/// duration: The desired duration of the output audio in seconds. If not
/// provided, it will be determined automatically.
/// apply_enhancement: Whether to apply audio enhancement to the output.
/// Defaults to True.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamTtsOutputConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_enhancement: Option<bool>,
}
