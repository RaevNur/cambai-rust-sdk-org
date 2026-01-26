pub use crate::prelude::*;

/// Query parameters for getEndToEndDubbingStatus
///
/// Request type for the GetEndToEndDubbingStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEndToEndDubbingStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
