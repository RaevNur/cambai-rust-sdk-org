pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTextToVoiceRequestPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    /// The text to be converted to voice
    pub text: String,
    /// Description to use for the created voice
    pub voice_description: String,
}
