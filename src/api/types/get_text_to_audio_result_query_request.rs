pub use crate::prelude::*;

/// Query parameters for getTextToAudioResult
///
/// Request type for the GetTextToAudioResultQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTextToAudioResultQueryRequest {
    /// Output format for the Text To Speech result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
}
