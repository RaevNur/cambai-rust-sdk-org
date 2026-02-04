pub use crate::prelude::*;

/// Query parameters for getStoryRunInfo
///
/// Request type for the GetStoryRunInfoQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetStoryRunInfoQueryRequest {
    /// Whether to include the transcription in the response for fetching the result of a Stories run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transcript: Option<Option<bool>>,
}
