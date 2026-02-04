pub use crate::prelude::*;

/// Query parameters for getTextToVoiceStatus
///
/// Request type for the GetTextToVoiceStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTextToVoiceStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
