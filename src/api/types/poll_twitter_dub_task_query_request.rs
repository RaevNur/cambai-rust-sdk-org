pub use crate::prelude::*;

/// Query parameters for pollTwitterDubTask
///
/// Request type for the PollTwitterDubTaskQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PollTwitterDubTaskQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
