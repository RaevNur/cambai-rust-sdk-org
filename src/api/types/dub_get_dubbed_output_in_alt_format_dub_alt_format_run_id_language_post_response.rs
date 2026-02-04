pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetDubbedOutputInAltFormatDubAltFormatRunIDLanguagePostResponse {
    Map0(HashMap<String, String>),

    OrchestratorPipelineCallResult(OrchestratorPipelineCallResult),
}

impl GetDubbedOutputInAltFormatDubAltFormatRunIDLanguagePostResponse {
    pub fn is_map0(&self) -> bool {
        matches!(self, Self::Map0(_))
    }

    pub fn is_orchestratorpipelinecallresult(&self) -> bool {
        matches!(self, Self::OrchestratorPipelineCallResult(_))
    }

    pub fn as_map0(&self) -> Option<&HashMap<String, String>> {
        match self {
            Self::Map0(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_map0(self) -> Option<HashMap<String, String>> {
        match self {
            Self::Map0(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_orchestratorpipelinecallresult(&self) -> Option<&OrchestratorPipelineCallResult> {
        match self {
            Self::OrchestratorPipelineCallResult(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_orchestratorpipelinecallresult(self) -> Option<OrchestratorPipelineCallResult> {
        match self {
            Self::OrchestratorPipelineCallResult(value) => Some(value),
            _ => None,
        }
    }
}
