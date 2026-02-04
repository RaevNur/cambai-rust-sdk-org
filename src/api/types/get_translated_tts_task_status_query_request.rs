pub use crate::prelude::*;

/// Query parameters for getTranslatedTtsTaskStatus
///
/// Request type for the GetTranslatedTtsTaskStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTranslatedTtsTaskStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
