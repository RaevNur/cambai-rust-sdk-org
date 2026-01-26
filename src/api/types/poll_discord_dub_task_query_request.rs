pub use crate::prelude::*;

/// Query parameters for pollDiscordDubTask
///
/// Request type for the PollDiscordDubTaskQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PollDiscordDubTaskQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
