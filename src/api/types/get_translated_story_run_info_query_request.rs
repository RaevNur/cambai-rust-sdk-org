pub use crate::prelude::*;

/// Query parameters for getTranslatedStoryRunInfo
///
/// Request type for the GetTranslatedStoryRunInfoQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTranslatedStoryRunInfoQueryRequest {
    /// Whether to include the transcription in the response for fetching the result of a Stories Translation run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transcript: Option<Option<bool>>,
}
