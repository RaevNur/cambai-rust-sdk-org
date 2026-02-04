pub use crate::prelude::*;

/// Query parameters for getDubbedOutputInAltFormatStatus
///
/// Request type for the GetDubbedOutputInAltFormatStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDubbedOutputInAltFormatStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
