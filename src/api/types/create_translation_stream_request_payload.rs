pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTranslationStreamRequestPayload {
    pub source_language: Languages,
    pub target_language: Languages,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formality: Option<Option<Formalities>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<Gender>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
}
