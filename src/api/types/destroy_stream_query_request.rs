pub use crate::prelude::*;

/// Query parameters for destroyStream
///
/// Request type for the DestroyStreamQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DestroyStreamQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
