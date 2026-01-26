pub use crate::prelude::*;

/// Query parameters for stopStream
///
/// Request type for the StopStreamQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StopStreamQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
