pub use crate::prelude::*;

/// Request for getTranscriptionResults (body + query parameters)
///
/// Request type for the GetTranscriptionResultsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetTranscriptionResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: RunIDsRequestPayload,
}
