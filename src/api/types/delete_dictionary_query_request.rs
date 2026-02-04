pub use crate::prelude::*;

/// Query parameters for deleteDictionary
///
/// Request type for the DeleteDictionaryQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDictionaryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
