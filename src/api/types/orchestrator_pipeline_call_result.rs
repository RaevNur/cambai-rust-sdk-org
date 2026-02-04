pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrchestratorPipelineCallResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}
