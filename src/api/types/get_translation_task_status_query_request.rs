pub use crate::prelude::*;

/// Query parameters for getTranslationTaskStatus
///
/// Request type for the GetTranslationTaskStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTranslationTaskStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
