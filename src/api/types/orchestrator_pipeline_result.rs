pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrchestratorPipelineResult {
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_reason: Option<Option<OrchestratorPipelineResultExceptionReason>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<OrchestratorPipelineResultMessage>>,
}
