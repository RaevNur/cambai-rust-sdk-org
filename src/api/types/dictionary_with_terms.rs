pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DictionaryWithTerms {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<DateTime<FixedOffset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_at: Option<Option<DateTime<FixedOffset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictionary_terms: Option<Option<Vec<DictionaryTerm>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<Languages>>>,
}
