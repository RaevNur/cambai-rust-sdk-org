pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Voice {
    pub id: i64,
    pub voice_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<Languages>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_url: Option<Option<String>>,
}
