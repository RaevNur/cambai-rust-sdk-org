pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStreamDataRequestPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Option<DateTime<FixedOffset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<Option<DateTime<FixedOffset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Option<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_languages: Option<Option<Vec<Languages>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
