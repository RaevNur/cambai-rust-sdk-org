pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SetupStoryStorySetupPostResponse {
    OrchestratorPipelineCallResult(OrchestratorPipelineCallResult),

    GetSetupStoryResultResponse(GetSetupStoryResultResponse),
}

impl SetupStoryStorySetupPostResponse {
    pub fn is_orchestratorpipelinecallresult(&self) -> bool {
        matches!(self, Self::OrchestratorPipelineCallResult(_))
    }

    pub fn is_getsetupstoryresultresponse(&self) -> bool {
        matches!(self, Self::GetSetupStoryResultResponse(_))
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

    pub fn as_getsetupstoryresultresponse(&self) -> Option<&GetSetupStoryResultResponse> {
        match self {
            Self::GetSetupStoryResultResponse(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_getsetupstoryresultresponse(self) -> Option<GetSetupStoryResultResponse> {
        match self {
            Self::GetSetupStoryResultResponse(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for SetupStoryStorySetupPostResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OrchestratorPipelineCallResult(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::GetSetupStoryResultResponse(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
