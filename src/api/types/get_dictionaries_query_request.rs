pub use crate::prelude::*;

/// Query parameters for getDictionaries
///
/// Request type for the GetDictionariesQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDictionariesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
