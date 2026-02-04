pub use crate::prelude::*;

/// Query parameters for getDictionaryDetails
///
/// Request type for the GetDictionaryDetailsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDictionaryDetailsQueryRequest {
    /// Limit how many terms are returned when fetching the dictionary details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_term: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
