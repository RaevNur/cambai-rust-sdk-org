pub use crate::prelude::*;

/// Request for getStoriesRunsResults (body + query parameters)
///
/// Request type for the GetStoriesRunsResultsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetStoriesRunsResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: RunIDsRequestPayload,
}
