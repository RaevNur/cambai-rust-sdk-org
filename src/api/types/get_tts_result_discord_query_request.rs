pub use crate::prelude::*;

/// Query parameters for getTtsResultDiscord
///
/// Request type for the GetTtsResultDiscordQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTtsResultDiscordQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
