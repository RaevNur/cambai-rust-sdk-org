pub use crate::prelude::*;

/// Request for getTranslationResults (body + query parameters)
///
/// Request type for the GetTranslationResultsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetTranslationResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: RunIDsRequestPayload,
}
