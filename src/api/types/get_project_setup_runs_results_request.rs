pub use crate::prelude::*;

/// Request for getProjectSetupRunsResults (body + query parameters)
///
/// Request type for the GetProjectSetupRunsResultsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetProjectSetupRunsResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: RunIDsRequestPayload,
}
