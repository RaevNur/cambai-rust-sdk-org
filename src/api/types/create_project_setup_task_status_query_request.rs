pub use crate::prelude::*;

/// Query parameters for createProjectSetupTaskStatus
///
/// Request type for the CreateProjectSetupTaskStatusQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProjectSetupTaskStatusQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
}
