pub use crate::prelude::*;

/// Query parameters for getTranscriptionResult
///
/// Request type for the GetTranscriptionResultQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTranscriptionResultQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_level_timestamps: Option<Option<bool>>,
}
