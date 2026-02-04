pub use crate::prelude::*;

/// Request for getAudioSeparationRunsResults (body + query parameters)
///
/// Request type for the GetAudioSeparationRunsResultsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAudioSeparationRunsResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: RunIDsRequestPayload,
}
