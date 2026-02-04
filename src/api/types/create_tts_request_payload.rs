pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTtsRequestPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    pub text: String,
    pub voice_id: i64,
    pub language: Languages,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
