pub use crate::prelude::*;

/// Request for getDubbingRunsResults (body + query parameters)
///
/// Request type for the GetDubbingRunsResultsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetDubbingRunsResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: RunIDsRequestPayload,
}
