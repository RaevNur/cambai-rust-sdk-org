pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StoryDetails {
    pub story_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_description: Option<Option<String>>,
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    pub source_language: Languages,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_languages: Option<Option<Vec<Languages>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Option<i64>>,
    pub studio_url: String,
}
