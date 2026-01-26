pub use crate::prelude::*;

/// Query parameters for getTtsRunInfo
///
/// Request type for the GetTtsRunInfoQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTtsRunInfoQueryRequest {
    /// Output format for the Text To Speech result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
}
