pub use crate::prelude::*;

/// Query parameters for listFolders
///
/// Request type for the ListFoldersQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFoldersQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_query: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
