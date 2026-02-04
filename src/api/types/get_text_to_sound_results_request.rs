pub use crate::prelude::*;

/// Request for getTextToSoundResults (body + query parameters)
///
/// Request type for the GetTextToSoundResultsRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetTextToSoundResultsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: RunIDsRequestPayload,
}
