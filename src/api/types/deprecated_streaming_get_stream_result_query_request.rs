pub use crate::prelude::*;

/// Query parameters for getStreamResult
///
/// Request type for the DeprecatedStreamingGetStreamResultQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeprecatedStreamingGetStreamResultQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
