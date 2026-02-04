pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectDetails {
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<Option<String>>,
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    pub source_language: Languages,
    pub target_languages: Vec<Languages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    pub studio_url: String,
}
