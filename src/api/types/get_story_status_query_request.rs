pub use crate::prelude::*;

/// Query parameters for getStoryStatus
///
/// Request type for the GetStoryStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetStoryStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
