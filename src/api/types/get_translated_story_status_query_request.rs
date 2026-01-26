pub use crate::prelude::*;

/// Query parameters for getTranslatedStoryStatus
///
/// Request type for the GetTranslatedStoryStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTranslatedStoryStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
