pub use crate::prelude::*;

/// Query parameters for getSourceLanguages
///
/// Request type for the GetSourceLanguagesQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSourceLanguagesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
