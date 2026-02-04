pub use crate::prelude::*;

/// Request for getProbeStream (body + query parameters)
///
/// Request type for the GetProbeStreamRequest2 operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetProbeStreamRequest2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    pub body: GetProbeStreamIn,
}
