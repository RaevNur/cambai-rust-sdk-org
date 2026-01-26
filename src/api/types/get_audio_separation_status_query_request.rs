pub use crate::prelude::*;

/// Query parameters for getAudioSeparationStatus
///
/// Request type for the GetAudioSeparationStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAudioSeparationStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
